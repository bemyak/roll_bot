use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;

use ejdb::bson;

use crate::db::LogMessage;
use crate::get_unix_time;
use crate::PROJECT_URL;

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

pub fn format_message_stats(msgs: Vec<LogMessage>) -> Result<String, Box<dyn Error>> {
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

pub fn help_message() -> String {
    format!("Hi! I'm a bot. The Dungeon Bot!
I can help you with your Dungeons&Dragons game (5th edition). I can:

/roll - roll a die. By default I have d20, but you can give me any number of dices! ex.: `/roll 2d6 +5`

/mm - search for a monster. I'll look in every book in Candlekeep and find at least one. ex.: `/mm tarasque`

/spell - search for a spell. I'll ask Elminster personally about it. ex.: `/spell fireball`

/item - search for an item. I'll cast Legend Lore spell to know what it is. ex.: `/item bag of holding`

My code is open like your brain to a Mind Flayer!
You can get it [here]({}) (code, not brain)
Suggestions and contributions are welcome.", PROJECT_URL)
}
