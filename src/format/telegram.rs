use teloxide::types::ChatKind;

use crate::PROJECT_URL;

pub fn chat_type_to_string(chat_type: &ChatKind) -> &'static str {
    match chat_type {
        ChatKind::Public(c) => match c.kind {
            teloxide::types::PublicChatKind::Channel(_) => "channel",
            teloxide::types::PublicChatKind::Group(_) => "group",
            teloxide::types::PublicChatKind::Supergroup(_) => "supergroup",
        },
        ChatKind::Private(c) => "private",
        // MessageChat::Private(_) => "private",
        // MessageChat::Group(_) => "group",
        // MessageChat::Supergroup(_) => "supergroup",
        // MessageChat::Unknown(_) => "unknown",
    }
}

pub fn help_message() -> String {
    format!("Hi! I'm a bot. The Dungeon Bot!
I can help you with your Dungeons&Dragons game (5th edition). I can:

/roll (or /r) - roll a die. By default I'll use d20, but you can give me any number of dices! e.g.: `/roll 2d6 +5`
run `/help roll` to know the secrets of the command

/monster (or /m) - search for a monster. I'll look in every book in Candlekeep and find at least one. e.g.: `/monster tarasque`

/spell (or /s) - search for a spell. I'll ask Elminster personally about it. e.g.: `/spell fireball`

/item (or /i) - search for an item. I'll cast Legend Lore spell to know what it is. e.g.: `/item bag of holding`

My code is open like your brain to a Mind Flayer!
You can get it [here]({}) (code, not brain)
Suggestions and contributions are welcome.", PROJECT_URL)
}

pub fn help_roll_message() -> &'static str {
    r#"Consider an example:
```
/roll 1d20 + 5 longsword
```
`1` is a number of dices to throw. It can be any number OR `+` OR `-` to indicate advantage or disadvantage respectfully.
`d20` is, as you may guess, a dice to throw. `20` indicates how many faces it has, can be any number.
`5` is a "plain value", but it also could be another die, e.g. `1d4`
`longsword` is a comment for the roll.

Any part of this expression can be omitted, so all of these are valid expressions:
`/r` → 1d20
`/r 2` → 2d20
`/r +` → +d20 (d20 with advantage)
`/r -` → -d20 (d20 with disadvantage
`/r d4` → 1d4
`/r +5` → 1d20+5
`/r sneak` → 1d20 with "sneak" comment

You can also combine multiple rolls in one command:
```
/r +5 longsword 3d6 piercing
```"#
}
