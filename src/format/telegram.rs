use telegram_bot::MessageChat;

use crate::PROJECT_URL;

pub fn chat_type_to_string(chat_type: &MessageChat) -> &'static str {
    match chat_type {
        MessageChat::Private(_) => "private",
        MessageChat::Group(_) => "group",
        MessageChat::Supergroup(_) => "supergroup",
        MessageChat::Unknown(_) => "unknown",
    }
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
