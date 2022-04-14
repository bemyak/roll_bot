use std::{borrow::Cow, env, time::Instant};

use ejdb::bson_crate::{ordered::OrderedDocument, Bson};
use inflector::Inflector;
use regex::{Captures, Regex};
use reqwest::Url;
use thiserror::Error;

use teloxide::{
    adaptors::{throttle::Limits, CacheMe, Throttle},
    dispatching2::UpdateFilterExt,
    prelude2::*,
    types::{
        ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, ReplyMarkup, Update,
        User,
    },
    utils::command::{BotCommand, ParseError},
    RequestError,
};

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
        error!("You must provide <code>ROLL_BOT_TOKEN</code> environment variable!");
        std::process::exit(1)
    });

    let bot = Bot::new(token)
        .cache_me()
        .throttle(Limits::default())
        .auto_send();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<RollBotCommand>()
                        .endpoint(process_command),
                )
                .branch(
                    dptree::filter(|msg: Message| {
                        msg.text().is_some() && msg.reply_to_message().is_some()
                    })
                    .endpoint(process_message),
                ),
        )
        .branch(Update::filter_callback_query().endpoint(process_callback_query));

    Dispatcher::builder(bot, handler)
        .build()
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
    EntryFormat(String),

    #[error("Bad reply")]
    BadReply(String),

    #[error("No reply text was produced")]
    NoReplyText(String),
}

async fn process_message(msg: Message, bot: RollBot) -> Result<(), BotError> {
    let (collection, item_name) = extract_search_data_from_reply(&msg)
        .ok_or_else(|| BotError::BadReply(msg.text().unwrap_or_default().to_owned()))?;
    search_item(msg.clone(), bot, collection, item_name).await?;
    Ok(())
}

fn extract_search_data_from_reply(msg: &Message) -> Option<(&'static Collection, &str)> {
    let item_name = msg.text()?;
    let reply = msg.reply_to_message()?;
    let reply = reply.text()?;
    // reply_data contains our own message generated in `search_item` function, e.g.: "What item should I look for? ..."
    // The second word is our collection name
    let mut iter = reply.split_whitespace();
    let _ = iter.next();

    let collection = iter.next()?;
    let collection = COMMANDS.get(collection)?;
    Some((collection, item_name))
}

async fn process_command(msg: Message, bot: RollBot, cmd: RollBotCommand) -> Result<(), BotError> {
    let start_processing = Instant::now();
    let chat_id = msg.chat.id;
    let chat_kind = msg.chat.kind.clone();
    let msg_text = msg.text().unwrap_or_default().to_string();

    trace!(
        "Got message from @{}: {}",
        msg.from()
            .map(User::full_name)
            .unwrap_or_else(|| "unknown".to_string()),
        msg.text().unwrap_or_default()
    );
    let response = match cmd {
        RollBotCommand::Help(opts) => print_help(msg, bot, opts).await,
        RollBotCommand::Roll(roll) => split_and_send(msg, bot, &roll, None).await,
        RollBotCommand::Stats => bot
            .send_message(msg.chat.id, stats()?)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(BotError::Request),
        RollBotCommand::Query((collection, item)) => search_item(msg, bot, collection, &item).await,
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

async fn print_help(msg: Message, bot: RollBot, opts: HelpOptions) -> Result<Message, BotError> {
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
            bot.send_message(msg.chat.id, format::telegram::help_message())
                .parse_mode(ParseMode::Html)
                .reply_markup(ReplyMarkup::InlineKeyboard(kb))
                .disable_web_page_preview(true)
                .await
                .map_err(BotError::Request)
        }
        HelpOptions::Roll => bot
            .send_message(msg.chat.id, format::telegram::help_roll_message())
            .parse_mode(ParseMode::Html)
            .await
            .map_err(BotError::Request),
    }
}

async fn process_callback_query(msg: CallbackQuery, bot: RollBot) -> Result<(), BotError> {
    trace!("Got callback from @{}: {:?}", msg.from.first_name, msg.data);
    if let (Some(data), Some(msg)) = (msg.data, msg.message) {
        // Support for old buttons, remove after a while
        let data = if data.starts_with('/') {
            data
        } else {
            "/".to_string() + &data
        };

        let bot_name = bot
            .get_me()
            .await
            .expect("Should always be successful")
            .user
            .first_name;
        let cmd = RollBotCommand::parse(&data, bot_name)?;

        process_command(msg, bot, cmd).await
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
        "<b>Table stats</b>\n{}\n\n<b>Usage stats</b> (since last month / total)\n{}\n\nLast database update <code>{}</code> ago",
        format_collection_metadata(collection_metadata),
        format_message_stats(messages)?,
        update_str,
    );
    Ok(msg)
}

async fn search_item(
    msg: Message,
    bot: RollBot,
    lookup_item: &Collection,
    arg: &str,
) -> Result<Message, BotError> {
    if arg.is_empty() {
        let force_reply = ForceReply::new()
            .selective(true)
            .input_field_placeholder(format!(
                "{} to search",
                lookup_item.get_default_command().to_title_case()
            ));

        return bot
            .send_message(
                msg.chat.id,
                format!(
                    "What {} should I look for? Please, <b>reply</b> to this message with a name:",
                    lookup_item.get_default_command()
                ),
            )
            .parse_mode(ParseMode::Html)
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
            let mut reply_msg = match lookup_item.type_ {
                crate::collection::CollectionType::Item => item.format_item(),
                crate::collection::CollectionType::Monster => item.format_monster(),
                crate::collection::CollectionType::Spell => item.format_spell(),
            }
            .ok_or_else(|| {
                BotError::EntryFormat(lookup_item.get_default_command().to_owned() + ": " + arg)
            })?;
            replace_string_links(&mut reply_msg, &mut keyboard);
            split_and_send(
                msg,
                bot,
                &reply_msg,
                Some(ReplyMarkup::InlineKeyboard(keyboard)),
            )
            .await
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

            let mut reply_msg = if keyboard.inline_keyboard.is_empty() {
                format!(
                    "Can't find any {} with this name, sorry :(",
                    lookup_item.get_default_command()
                )
            } else {
                format!(
                    "I don't have any {} with this exact name, but these look similar:",
                    lookup_item.get_default_command()
                )
            };

            replace_string_links(&mut reply_msg, &mut keyboard);
            split_and_send(
                msg,
                bot,
                &reply_msg,
                Some(ReplyMarkup::InlineKeyboard(keyboard)),
            )
            .await
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
                Some(bonus) => format!("<b>{}</b>", bonus.as_str()),
                None => "".to_owned(),
            },
            "atk" => "".to_owned(),
            "scaledamage" => format!("<b>{}</b>", nice_str),
            "dice" | "damage" => {
                let roll_results = roll_results(arg1).unwrap();
                let roll = roll_results.get(0).unwrap();
                format!("<b>{}</b> <code>[{}]</code> ", arg1, roll.expression.calc())
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
                format!("<i>{}</i>", nice_str)
            }
        }
    });

    std::mem::swap(text, &mut result.into_owned());
}

// Splitting messages that are too long
// Hope this will be moved into the tg lib someday

async fn split_and_send(
    msg: Message,
    bot: RollBot,
    text: &str,
    keyboard: Option<ReplyMarkup>,
) -> Result<Message, BotError> {
    if text.is_empty() {
        return Err(BotError::NoReplyText(
            msg.text().unwrap_or_default().to_owned(),
        ));
    }

    let messages = split2(text, 4096);
    let (last, all) = messages.split_last().unwrap();

    for text in all {
        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
    }

    let mut answer = bot
        .send_message(msg.chat.id, last)
        .parse_mode(ParseMode::Html);

    if let Some(markup) = keyboard {
        answer = answer.reply_markup(markup);
    }

    answer.await.map_err(BotError::Request)
}

fn get_margin(v: &[Vec<u8>]) -> usize {
    v.iter().map(|s| s.len() + 3).sum()
}

fn split2(text: &str, max_len: usize) -> Vec<String> {
    enum State {
        Text,
        OpenTag,
        CloseTag,
    }

    let mut result = Vec::new();
    let mut part = Vec::new();
    let mut state = State::Text;
    let mut tags = Vec::new();
    let mut tag = Vec::new();
    let mut sep_pos = 0;
    let mut sep_tag_len = 0;

    for b in text.bytes() {
        if part.len() >= max_len - get_margin(&tags[..sep_tag_len]) {
            if sep_pos == 0 {
                sep_tag_len = tags.len();
                sep_pos = part.len();
            }
            let mut new_part: Vec<_> = part.drain(..sep_pos).collect();
            for tag in tags[..sep_tag_len].iter().rev() {
                new_part.extend(format!("</{}>", String::from_utf8_lossy(tag)).as_bytes());
            }
            result.push(String::from_utf8_lossy(&new_part).to_string());
            sep_pos = 0;

            if !part.is_empty() {
                part.remove(0);
            }
            let mut closing_tags = tags[..sep_tag_len]
                .iter()
                .map(|t| format!("<{}>", String::from_utf8_lossy(t)))
                .collect::<Vec<_>>()
                .join("")
                .into_bytes();
            closing_tags.extend(&part);
            part = closing_tags;
        }

        match state {
            State::Text => match b as char {
                '<' => {
                    state = State::OpenTag;
                }
                '\n' => {
                    if part.is_empty() {
                        continue;
                    }
                    sep_tag_len = tags.len();
                    sep_pos = part.len();
                }
                _ => {}
            },
            State::OpenTag => match b as char {
                '/' => {
                    if tag.is_empty() {
                        state = State::CloseTag
                    } else {
                        tag.clear();
                        state = State::Text
                    }
                }
                '>' => {
                    tags.push(tag);
                    tag = Vec::new();
                    state = State::Text;
                }
                _ => tag.push(b),
            },
            State::CloseTag => match b as char {
                '>' => {
                    if Some(&tag) == tags.last() {
                        tags.pop();
                        if sep_tag_len != 0 {
                            sep_tag_len -= 1;
                        }
                    } else {
                        warn!(
                            "Unexpected closing tag: {} in\n{text}",
                            String::from_utf8_lossy(&tag)
                        )
                    }
                    tag = Vec::new();
                    state = State::Text;
                }
                _ => tag.push(b),
            },
        }
        part.push(b);
    }

    let part_str = String::from_utf8_lossy(&part);

    result.push(part_str.to_string());
    result
}

#[test]
fn test_split2_simple1() {
    let parts = split2("123\n456", 3);
    assert_eq!("123", parts[0]);
    assert_eq!("456", parts[1]);
}

#[test]
fn test_split2_simple2() {
    let parts = split2("123\n456", 4);
    assert_eq!("123", parts[0]);
    assert_eq!("456", parts[1]);
}

#[test]
fn test_split2_tags1() {
    let parts = split2("<b>1</b>23\n4<i>5</i>6", 17);
    assert_eq!("<b>1</b>23", parts[0]);
    assert_eq!("4<i>5</i>6", parts[1]);
}

#[test]
fn test_split2_tags2() {
    let parts = split2("<b>123\n4<i>5</i>6</b>", 17);
    assert_eq!("<b>123</b>", parts[0]);
    assert_eq!("<b>4<i>5</i>6</b>", parts[1]);
}
