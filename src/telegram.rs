#![allow(deprecated)]

use std::{borrow::Cow, cmp::min, env, time::Instant};

use ejdb::bson_crate::{ordered::OrderedDocument, Bson};
use regex::{Captures, Regex};
use reqwest::Url;
use thiserror::Error;

use teloxide::{
    adaptors::{throttle::Limits, CacheMe, Throttle},
    payloads::SendMessageSetters,
    prelude::*,
    types::{ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, ReplyMarkup, User},
    utils::command::{BotCommand, ParseError},
    RequestError,
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
        telegram::chat_type_to_string,
    },
    DB, DONATION_URL, PROJECT_URL,
};

type RollBot = AutoSend<Throttle<CacheMe<Bot>>>;

pub async fn start() {
    let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
        error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
        std::process::exit(1)
    });

    let bot = Bot::new(token)
        .cache_me()
        .throttle(Limits::default())
        .auto_send();
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
    // #[error("Telegram Error")]
    // Telegram(ApiError),
    #[error("Database Error")]
    Db(#[from] ejdb::Error),

    #[error("Die Format Error")]
    DieFormat(#[from] DieFormatError),

    #[error("Parse Error")]
    Parse(#[from] ParseError),

    #[error("Entry Format Error")]
    EntryFormat,
}

#[allow(deprecated)]
pub async fn process_message(
    cx: UpdateWithCx<RollBot, Message>,
    command: RollBotCommand,
) -> Result<(), BotError> {
    let start_processing = Instant::now();
    let chat_id = cx.update.chat_id();
    let chat_kind = cx.update.chat.kind.clone();
    let msg_text = cx.update.text().unwrap_or_default().to_string();

    trace!(
        "Got message from @{}: {}",
        cx.update
            .from()
            .map(User::full_name)
            .unwrap_or_else(|| "unknown".to_string()),
        cx.update.text().unwrap_or_default()
    );
    let response = match command {
        RollBotCommand::Help(opts) => print_help(cx, opts).await,
        RollBotCommand::Roll(roll) => split_and_send(cx, &roll, None).await,
        RollBotCommand::Stats => cx
            .answer(stats()?)
            .parse_mode(ParseMode::Markdown)
            .await
            .map_err(BotError::Request),
        RollBotCommand::Query((collection, item)) => search_item(cx, collection, &item).await,
    };

    DB.log_message(
        chat_id,
        chat_type_to_string(&chat_kind),
        msg_text,
        &response.map(|r| r.text().map(|s| s.to_owned())),
        Instant::now()
            .checked_duration_since(start_processing)
            .unwrap()
            .as_millis() as u64,
    );

    Ok(())
}

async fn print_help(
    cx: UpdateWithCx<RollBot, Message>,
    opts: HelpOptions,
) -> Result<Message, BotError> {
    match opts {
        HelpOptions::None => {
            let kb = InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::url(
                        "Source Code".into(),
                        Url::parse(PROJECT_URL).unwrap(),
                    ),
                    InlineKeyboardButton::url(
                        "Buy me a coffee".into(),
                        Url::parse(DONATION_URL).unwrap(),
                    ),
                ],
                vec![
                    InlineKeyboardButton::url(
                        "News".into(),
                        Url::parse("https://t.me/roll_bot_news").unwrap(),
                    ),
                    InlineKeyboardButton::url(
                        "Chat".into(),
                        Url::parse("https://t.me/roll_bot_chat").unwrap(),
                    ),
                ],
            ]);
            cx.answer(format::telegram::help_message())
                .parse_mode(ParseMode::Markdown)
                .reply_markup(ReplyMarkup::InlineKeyboard(kb))
                .disable_web_page_preview(true)
                .await
                .map_err(BotError::Request)
        }
        HelpOptions::Roll => cx
            .answer(format::telegram::help_roll_message())
            .parse_mode(ParseMode::Markdown)
            .await
            .map_err(BotError::Request),
    }
}

async fn process_callback_query(cx: UpdateWithCx<RollBot, CallbackQuery>) -> Result<(), BotError> {
    trace!(
        "Got callback from @{}: {:?}",
        cx.update.from.first_name,
        cx.update.data
    );
    if let (Some(data), Some(msg)) = (cx.update.data, cx.update.message) {
        // Support for old buttons, remove after a while
        let data = if data.starts_with('/') {
            data
        } else {
            "/".to_string() + &data
        };

        let bot_name = cx
            .requester
            .get_me()
            .await
            .expect("Should always be successful")
            .user
            .first_name;
        let cmd = RollBotCommand::parse(&data, bot_name)?;

        let new_cx = UpdateWithCx {
            requester: cx.requester.clone(),
            update: msg,
        };

        process_message(new_cx, cmd).await
    } else {
        Ok(())
    }
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
    cx: UpdateWithCx<RollBot, Message>,
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
            .map_err(BotError::Request);
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
            split_and_send(cx, &msg, Some(ReplyMarkup::InlineKeyboard(keyboard))).await
        }
        None => {
            let iter = lookup_item
                .collections
                .iter()
                .cloned()
                .flat_map(|collection| {
                    let cache = DB.cache.read().unwrap();
                    let engine = cache.get(collection).unwrap();
                    let results = engine.search(arg);
                    results.into_iter().map(|item| {
                        let command = format!("/{} {}", lookup_item.get_default_command(), item);
                        let button = InlineKeyboardButton::callback(item, command);
                        vec![button]
                    })
                })
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
            split_and_send(cx, &msg, Some(ReplyMarkup::InlineKeyboard(keyboard))).await
            // cx.answer(&msg)
            //     .parse_mode(ParseMode::Markdown)
            //     .reply_markup(ReplyMarkup::InlineKeyboard(keyboard))
            // .await
            // .map_err(BotError::Request)
        }
    }
}

fn replace_links(doc: &mut OrderedDocument, keyboard: &mut InlineKeyboardMarkup) {
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
            for val in arr {
                replace_bson_links(val, keyboard);
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
                }
                format!("_{}_", nice_str)
            }
        }
    });

    std::mem::swap(text, &mut result.into_owned());
}

// Splitting messages that are too long
// Hope this will be moved into the tg lib someday

#[allow(deprecated)]
async fn split_and_send(
    cx: UpdateWithCx<RollBot, Message>,
    text: &str,
    keyboard: Option<ReplyMarkup>,
) -> Result<Message, BotError> {
    if text.is_empty() {
        return Err(BotError::EntryFormat);
    }

    let messages = split(text);
    let (last, all) = messages.split_last().unwrap();

    for text in all {
        cx.answer(text).parse_mode(ParseMode::Markdown).await?;
    }

    let mut answer = cx.answer(last).parse_mode(ParseMode::Markdown);

    if let Some(markup) = keyboard {
        answer = answer.reply_markup(markup);
    }

    answer.await.map_err(BotError::Request)
}

fn split(text: &str) -> Vec<String> {
    const MAX_TEXT_LEN: usize = 4096;
    let mut result = Vec::new();

    let bytes = text.as_bytes();

    let mut start = 0;
    let mut end;

    while start < bytes.len() {
        let hard_end = min(start + MAX_TEXT_LEN, bytes.len());
        end = get_end(bytes, start, hard_end);

        // We already know that it is a valid utf8
        let s = unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) };
        result.push(s);
        start = end + 1;
    }

    fix_tags(&result)
}

fn get_end(bytes: &[u8], start: usize, hard_end: usize) -> usize {
    if hard_end == bytes.len() {
        return hard_end;
    }

    for (i, byte) in bytes[start..hard_end].iter().enumerate().rev() {
        let c = *byte as char;
        if c == '\n' {
            return start + i;
        }
    }
    hard_end
}

fn fix_tags(split: &[&str]) -> Vec<String> {
    const MARKUP_TAGS: [&str; 4] = ["```", "`", "*", "_"];

    if split.len() == 1 {
        return split.iter().map(|s| s.to_string()).collect();
    }

    let mut tag_to_fix: Option<&str> = None;
    split
        .iter()
        .map(|s| {
            let mut s = s.to_string();
            if let Some(t) = tag_to_fix {
                tag_to_fix = None;
                s = t.to_string() + &s;
            }
            for t in MARKUP_TAGS.iter() {
                if s.matches(t).count() % 2 != 0 {
                    s.push_str(t);
                    tag_to_fix = Some(t);
                }
            }
            s
        })
        .collect::<Vec<_>>()
}
