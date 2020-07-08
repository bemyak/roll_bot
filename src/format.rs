use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use ejdb::bson;
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use rand::prelude::*;
use regex::Regex;
use telegram_bot::MessageChat;
use thiserror::Error;

use crate::db::LogMessage;
use crate::get_unix_time;
use crate::PROJECT_URL;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Formatting error ocurred: {0}")]
    Other(String),
    #[error("Too many dices were requested")]
    TooManyDices,
}

pub fn format_document(doc: bson::Document) -> String {
    let mut res = String::new();
    doc.into_iter().for_each(|(k, v)| match k.as_ref() {
        "_id" => {}
        "name" => write!(&mut res, "**{}**\n\n", v).unwrap(),
        "entries" => {
            let s = match v {
                bson::Bson::Array(arr) => arr
                    .into_iter()
                    .map(|bs| simple_format(bs))
                    .collect::<Vec<String>>()
                    .join("\n\n"),
                _ => simple_format(v),
            };
            write!(&mut res, "\n{}\n\n", s).unwrap()
        }
        _ => write!(&mut res, "*{}*: {}\n", k, simple_format(v)).unwrap(),
    });
    res
}

fn simple_format(bs: bson::Bson) -> String {
    match bs {
        bson::Bson::FloatingPoint(num) => format!("{}", num),
        bson::Bson::String(s) => s,
        bson::Bson::Array(arr) => arr
            .into_iter()
            .map(|bs| simple_format(bs))
            .collect::<Vec<String>>()
            .join(", "),
        bson::Bson::Document(doc) => doc
            .into_iter()
            .map(|(k, v)| format!("{}: {}", k, simple_format(v)))
            .collect::<Vec<_>>()
            .join(", "),
        bson::Bson::Boolean(b) => match b {
            true => "Yes".to_owned(),
            false => "No".to_owned(),
        },
        bson::Bson::Null => "null".to_owned(),
        bson::Bson::I32(num) => format!("{}", num),
        bson::Bson::I64(num) => format!("{}", num),
        _ => panic!("Unknown type: {:?}", bs.element_type()),
    }
}

pub fn chat_type_to_string(chat_type: &MessageChat) -> &'static str {
    match chat_type {
        MessageChat::Private(_) => "private",
        MessageChat::Group(_) => "group",
        MessageChat::Supergroup(_) => "supergroup",
        MessageChat::Unknown(_) => "unknown",
    }
}

pub fn format_message_stats(msgs: Vec<LogMessage>) -> Result<String, ejdb::Error> {
    let now = get_unix_time();
    let mount_ago = now - 60 * 60 * 24 * 30;

    let msg_total = msgs.len();
    let msg_total_month = msgs.iter().filter(|msg| msg.timestamp >= mount_ago).count();

    let max_latency = msgs.iter().map(|msg| msg.latency).max().unwrap_or_default() as usize;
    let max_latency_month = msgs
        .iter()
        .filter(|msg| msg.timestamp >= mount_ago)
        .map(|msg| msg.latency)
        .max()
        .unwrap_or_default() as usize;

    let min_latency = msgs.iter().map(|msg| msg.latency).min().unwrap_or_default() as usize;
    let min_latency_month = msgs
        .iter()
        .filter(|msg| msg.timestamp >= mount_ago)
        .map(|msg| msg.latency)
        .min()
        .unwrap_or_default() as usize;

    let avg_latency = match msg_total {
        0 => 0,
        _ => msgs.iter().fold(0, |acc, msg| acc + msg.latency) as usize / msg_total,
    };

    let avg_latency_month = match msg_total_month {
        0 => 0,
        _ => {
            msgs.iter()
                .filter(|msg| msg.timestamp >= mount_ago)
                .fold(0, |acc, msg| acc + msg.latency) as usize
                / msg_total_month
        }
    };

    let users: HashMap<i64, u64> = {
        let mut users = HashMap::new();

        msgs.iter().for_each(|msg| {
            let old_ts = users.get(&msg.user_id);
            match old_ts {
                None => {
                    users.insert(msg.user_id, msg.timestamp);
                }
                Some(old_ts) => {
                    if old_ts < &msg.timestamp {
                        users.insert(msg.user_id, msg.timestamp);
                    }
                }
            }
        });
        users
    };

    let users_total = users.iter().count();
    let users_total_month = users.iter().filter(|(_, ts)| ts >= &&mount_ago).count();

    Ok(format!(
        "Total messages: `{}` / `{}`
Unique users: `{}` / `{}`
Max latency, ms: `{}` / `{}`
Avg latency, ms: `{}` / `{}`
Min latency, ms: `{}` / `{}`",
        msg_total_month,
        msg_total,
        users_total_month,
        users_total,
        max_latency_month,
        max_latency,
        avg_latency_month,
        avg_latency,
        min_latency_month,
        min_latency
    ))
}

pub fn format_collection_metadata(meta: ejdb::meta::DatabaseMetadata) -> String {
    meta.collections()
        .map(|col| format!("`{}`: `{}` records", col.name(), col.records()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug)]
pub enum RollMode {
    ADV,
    DADV,
    NORMAL,
}

#[derive(Debug)]
pub struct Roll {
    pub die: String,
    pub roll_results: Vec<i32>,
    pub total: i32,
    pub mode: RollMode,
    pub num: i32,
    pub face: i32,
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let roll_results = self
            .roll_results
            .iter()
            .map(|roll| {
                if *roll == 1 || *roll == self.face {
                    format!("*{}*", roll)
                } else {
                    format!("{}", roll)
                }
            })
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "`{}:` \\[{}] = *{}*", self.die, roll_results, self.total)
    }
}

pub fn roll_dice(msg: &str) -> Result<String, FormatError> {
    let response = roll_results(msg)?
        .iter()
        .map(|roll| roll.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if response.len() == 0 {
        warn!("Cannot parse: {}", msg);
        Ok("Err, sorry, I can't roll that. Maybe you need some /help ?".to_owned())
    } else {
        Ok(response)
    }
}

pub fn roll_results(msg: &str) -> Result<Vec<Roll>, FormatError> {
    lazy_static! {
        static ref DICE_REGEX: Regex = Regex::new(r"(?P<num>\+|\-|\d+)?(?:(?:d|к|д)(?P<face>\d+))?\s*(?:(?P<bonus_sign>\+|\-|\*|/)\s*(?P<bonus_value>\d+))?").unwrap();
    }

    let mut result = Vec::new();

    let iter = DICE_REGEX.captures_iter(msg);

    if iter.size_hint().0 > 100 {
        return Err(FormatError::TooManyDices);
    }

    for cap in iter {
        if msg != ""
            && cap
                .name("num")
                .or(cap.name("face"))
                .or(cap.name("bonus_sign"))
                .or(cap.name("bonus_value"))
                == None
        {
            continue;
        }

        let num = cap.name("num").map_or("1", |m| m.as_str());
        let face: i32 = cap
            .name("face")
            .map(|m| FromStr::from_str(m.as_str()).ok())
            .flatten()
            .unwrap_or(20);
        let bonus_sign = cap.name("bonus_sign").map(|m| m.as_str());
        let bonus_value: Option<i32> = cap
            .name("bonus_value")
            .map(|m| FromStr::from_str(m.as_str()).ok())
            .flatten();

        let (mode, capacity) = match num {
            "+" => (RollMode::ADV, 2),
            "-" => (RollMode::DADV, 2),
            _ => (
                RollMode::NORMAL,
                FromStr::from_str(num).map_err(|err| {
                    FormatError::Other(format!("Cannot parse roll expression: {}", err))
                })?,
            ),
        };

        if capacity > 300 {
            return Err(FormatError::TooManyDices);
        }

        let roll_results: Vec<i32> = (0..capacity)
            .map(|_| rand::thread_rng().gen_range(0, face) + 1)
            .collect();

        let mut total: i32 = match mode {
            RollMode::ADV => *roll_results.iter().max().unwrap_or(&0),
            RollMode::DADV => *roll_results.iter().min().unwrap_or(&0),
            RollMode::NORMAL => roll_results.iter().sum(),
        };

        let mut die = match mode {
            RollMode::ADV => format!("d{} with advantage", face),
            RollMode::DADV => format!("d{} with disadvantage", face),
            RollMode::NORMAL => format!("{}d{}", num, face),
        };

        if let (Some(bonus_sign), Some(bonus_value)) = (bonus_sign, bonus_value) {
            die.push_str(&format!(" {} {}", bonus_sign, bonus_value));
            match bonus_sign {
                "+" => total += bonus_value,
                "-" => total -= bonus_value,
                "*" => total *= bonus_value,
                "/" => total /= bonus_value,
                other => {
                    let err = FormatError::Other(format!(
                        "Cannot parse roll expression: unknown symbol {}",
                        other
                    ));
                    error!("{} in message: {}", err, msg);
                    return Err(err);
                }
            }
        }

        result.push(Roll {
            die,
            total,
            roll_results,
            mode,
            face,
            num: capacity,
        })
    }

    Ok(result)
}

pub fn help_message() -> String {
    format!("Hi! I'm a bot. The Dungeon Bot!
I can help you with your Dungeons&Dragons game (5th edition). I can:

/roll (or /r) - roll a die. By default I'll use d20, but you can give me any number of dices! e.g.: `/roll 2d6 +5`

/monster (or /m) - search for a monster. I'll look in every book in Candlekeep and find at least one. e.g.: `/monster tarasque`

/spell (or /s) - search for a spell. I'll ask Elminster personally about it. e.g.: `/spell fireball`

/item (or /i) - search for an item. I'll cast Legend Lore spell to know what it is. e.g.: `/item bag of holding`

My code is open like your brain to a Mind Flayer!
You can get it [here]({}) (code, not brain)
Suggestions and contributions are welcome.", PROJECT_URL)
}

// Telegram allows only alphanumeric characters and underscores in bot commands,
// but we often have spaces, apostrophes and so on.
// Here we are utilizing url encode to replace such symbols with %\d\d (e.g., %20),
// but since % is itself forbidden character we are wrapping the resulting digit in underscores.
pub fn tg_encode(msg: &str) -> String {
    lazy_static! {
        static ref CHAR_REGEX: Regex = Regex::new(r"%([\dA-F]{2})").unwrap();
    }
    let msg = utf8_percent_encode(msg, NON_ALPHANUMERIC).to_string();
    CHAR_REGEX.replace_all(&msg, "_${1}_").into()
}

pub fn tg_decode(msg: &str) -> String {
    lazy_static! {
        static ref CHAR_REGEX: Regex = Regex::new(r"_([\dA-F]{2})_").unwrap();
    }
    let url_encoded = CHAR_REGEX.replace_all(&msg, "%${1}");
    percent_decode(url_encoded.as_bytes())
        .decode_utf8_lossy()
        .to_string()
}
