use std::{borrow::Cow, env, time::Instant};

use ejdb::bson_crate::{ordered::OrderedDocument, Bson};
use regex::{Captures, Regex};
use thiserror::Error;

use teloxide::{
    adaptors::{throttle::Limits, Throttle},
    dispatching::update_listeners,
    payloads::SendMessageSetters,
    prelude::*,
    types::{ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, ReplyMarkup},
    ApiError, RequestError,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    collection::{Collection, COMMANDS},
    commands::{HelpOptions, RollBotCommand},
    format::{
        self,
        db::{format_collection_metadata, format_message_stats},
        item::Item,
        monster::Monster,
        roll::{roll_results, DieFormatError},
        spell::Spell,
    },
    DB,
};

pub async fn start() {
    let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
        error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
        std::process::exit(1)
    });

    let bot = Bot::new(token).throttle(Limits::default()).auto_send();
    let bot_name = bot
        .get_me()
        .await
        .expect("Should always be successful")
        .user
        .first_name;

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<_, Message>| {
            UnboundedReceiverStream::new(rx)
                .commands(bot_name)
                .for_each_concurrent(None, |(cx, cmd)| async {
                    process_message(cx, cmd).await.log_on_error().await;
                })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<_, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |message| async {
                process_callback_query(message).await.log_on_error().await;
            })
        })
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Request Error")]
    Request(#[from] RequestError),
    #[error("Telegram Error")]
    Telegram(ApiError),
    #[error("Database Error")]
    Db(#[from] ejdb::Error),

    #[error("Die Format Error")]
    DieFormat(#[from] DieFormatError),

    #[error("Entry Format Error")]
    EntryFormat,
}

pub async fn process_message(
    cx: UpdateWithCx<AutoSend<Throttle<Bot>>, Message>,
    command: RollBotCommand,
) -> Result<(), BotError> {
    match command {
        RollBotCommand::Help(opts) => match opts {
            HelpOptions::None => {
                cx.answer(format::telegram::help_message())
                    .parse_mode(ParseMode::Markdown)
                    .await?
            }
            HelpOptions::Roll => {
                cx.answer(format::telegram::help_roll_message())
                    .parse_mode(ParseMode::Markdown)
                    .await?
            }
        },
        RollBotCommand::Roll(roll) => cx.answer(roll).parse_mode(ParseMode::Markdown).await?,
        RollBotCommand::Stats => cx.answer(stats()?).parse_mode(ParseMode::Markdown).await?,
        RollBotCommand::Query((collection, item)) => search_item(cx, collection, &item).await?,
    };

    Ok(())
}

async fn process_callback_query(
    cx: UpdateWithCx<AutoSend<Throttle<Bot>>, CallbackQuery>,
) -> Result<(), BotError> {
    Ok(())
}

fn stats() -> Result<String, BotError> {
    let last_update = Instant::now()
        .checked_duration_since(DB.get_update_timestamp())
        .unwrap()
        .as_secs();

    let update_str = match last_update {
        0..=60 => format!("{}s", last_update),
        61..=3600 => format!("{}m", last_update / 60),
        3601..=86400 => format!("{}h", last_update / 60 / 60),
        86401..=std::u64::MAX => format!("{}d", last_update / 60 / 60 / 24),
    };

    let collection_metadata = DB.get_metadata()?;
    let messages = DB.get_all_massages()?;

    let msg = format!(
        "*Table stats*\n{}\n\n*Usage stats* (since last month / total)\n{}\n\nLast database update `{}` ago",
        format_collection_metadata(collection_metadata),
        format_message_stats(messages)?,
        update_str,
    );
    Ok(msg)
}

async fn search_item(
    cx: UpdateWithCx<AutoSend<Throttle<Bot>>, Message>,
    lookup_item: &Collection,
    arg: &str,
) -> Result<Message, BotError> {
    if arg.is_empty() {
        let force_reply = ForceReply::new().selective(true);

        return cx
            .answer(format!(
                "What {} should I look for? Please, *reply* on this message with a name:",
                lookup_item.get_default_command()
            ))
            .parse_mode(ParseMode::Markdown)
            .reply_markup(force_reply)
            .await
            .map_err(|err| BotError::Request(err));
    }

    let exact_match_result = lookup_item
        .collections
        .iter()
        .filter_map(|collection| DB.get_item(collection, arg).ok().flatten())
        .next();

    match exact_match_result {
        Some(mut item) => {
            let mut keyboard = InlineKeyboardMarkup::default();
            replace_links(&mut item, &mut keyboard);
            let mut msg = match lookup_item.type_ {
                crate::collection::CollectionType::Item => item.format_item(),
                crate::collection::CollectionType::Monster => item.format_monster(),
                crate::collection::CollectionType::Spell => item.format_spell(),
            }
            .ok_or(BotError::EntryFormat)?;
            replace_string_links(&mut msg, &mut keyboard);
            cx.answer(&msg)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(ReplyMarkup::InlineKeyboard(keyboard))
                .await
                .map_err(|err| BotError::Request(err))
        }
        None => {
            let iter = lookup_item
                .collections
                .iter()
                .cloned()
                .map(|collection| {
                    let cache = DB.cache.read().unwrap();
                    let engine = cache.get(collection).unwrap();
                    let results = engine.search(arg);
                    results.into_iter().map(|item| {
                        let command = format!("{} {}", lookup_item.get_default_command(), item);
                        let button = InlineKeyboardButton::callback(item, command);
                        vec![button]
                    })
                })
                .flatten()
                .collect::<Vec<_>>();

            let mut keyboard = InlineKeyboardMarkup::new(iter);

            let mut msg = if keyboard.inline_keyboard.is_empty() {
                format!(
                    "Can't find any {} with this name, sorry :(",
                    lookup_item.get_default_command()
                )
            } else {
                format!(
                    "I don't have any {} with this exact name, but these looks similar:",
                    lookup_item.get_default_command()
                )
            };

            replace_string_links(&mut msg, &mut keyboard);
            cx.answer(&msg)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(ReplyMarkup::InlineKeyboard(keyboard))
                .await
                .map_err(|err| BotError::Request(err))
        }
    }
}

pub fn replace_links(doc: &mut OrderedDocument, keyboard: &mut InlineKeyboardMarkup) {
    let keys: Vec<String> = doc.keys().cloned().collect();
    for key in keys {
        let entry = doc.entry(key).or_insert(Bson::String("".to_string()));
        replace_bson_links(entry, keyboard);
    }
}

fn replace_bson_links(b: &mut Bson, keyboard: &mut InlineKeyboardMarkup) {
    match b {
        Bson::String(val) => {
            replace_string_links(val, keyboard);
        }
        Bson::Array(arr) => {
            for mut val in arr {
                replace_bson_links(&mut val, keyboard);
            }
        }
        Bson::Document(doc) => {
            replace_links(doc, keyboard);
        }
        _ => {}
    }
}

fn replace_string_links(text: &mut String, keyboard: &mut InlineKeyboardMarkup) {
    lazy_static! {
        static ref LINK_REGEX: Regex =
            Regex::new(r"\{@(?P<cmd>\w+)(?:\s+(?P<arg1>.+?))?(?:\|(?P<arg2>.*?))?(?:\|(?P<arg3>.*?))?(?:\|(?P<arg4>.*?))?\}(?P<bonus>\d+)?")
                .unwrap();
    }

    let text_copy = text.clone();

    let result: Cow<str> = LINK_REGEX.replace_all(&text_copy, |caps: &Captures| {
        let cmd = caps.name("cmd").unwrap().as_str();
        let arg1 = caps
            .name("arg1")
            .map(|cap| cap.as_str())
            .unwrap_or_default();
        let arg2 = caps.name("arg2").map(|cap| cap.as_str());
        let arg3 = caps.name("arg3").map(|cap| cap.as_str());
        let arg4 = caps.name("arg4").map(|cap| cap.as_str());

        let nice_str = arg4.or(arg3).or(arg2).unwrap_or(arg1);

        match cmd {
            "i" => format!("• {}", nice_str),
            "hit" => format!("+{}", arg1),
            "h" => match caps.name("bonus") {
                Some(bonus) => format!("*{}*", bonus.as_str()),
                None => "".to_owned(),
            },
            "atk" => "".to_owned(),
            "scaledamage" => format!("*{}*", nice_str),
            "dice" | "damage" => {
                let roll_results = roll_results(arg1).unwrap();
                let roll = roll_results.get(0).unwrap();
                format!("*{}* `[{}]` ", arg1, roll.expression.calc())
            }
            "recharge" => {
                if arg1.is_empty() {
                    "(Recharge 6)".to_string()
                } else {
                    format!("(Recharge {}-6)", arg1)
                }
            }
            _ => {
                let nice_str = arg4.or(arg3).unwrap_or(arg1);
                if let Some(item) = COMMANDS.get(cmd) {
                    let mut kb = keyboard.clone();
                    kb = kb.append_row(vec![InlineKeyboardButton::callback(
                        format!("{}: {}", item.get_default_command(), nice_str),
                        format!("{} {}", item.get_default_command(), arg1),
                    )]);
                    *keyboard = kb;
                    format!("_{}_", nice_str)
                } else {
                    format!("_{}_", nice_str)
                }
                format!("_{}_", nice_str)
            }
        }
    });

    std::mem::swap(text, &mut result.into_owned());
}
