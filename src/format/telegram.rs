use teloxide::types::ChatKind;

use crate::PROJECT_URL;

pub fn chat_type_to_string(chat_type: &ChatKind) -> &'static str {
    match chat_type {
        ChatKind::Public(c) => match c.kind {
            teloxide::types::PublicChatKind::Channel(_) => "channel",
            teloxide::types::PublicChatKind::Group(_) => "group",
            teloxide::types::PublicChatKind::Supergroup(_) => "supergroup",
        },
        ChatKind::Private(_) => "private",
    }
}

pub fn help_message() -> String {
    format!("Hi! I'm a bot. The Dungeon Bot!
I can help you with your Dungeons&Dragons game (5th edition). I can:

/roll (or /r) - roll a die. By default I'll use d20, but you can give me any number of dices! e.g.: <code>/roll 2d6 +5</code>
run <code>/help roll</code> to know the secrets of the command

/monster (or /m) - search for a monster. I'll look in every book in Candlekeep and find at least one. e.g.: <code>/monster tarasque</code>

/spell (or /s) - search for a spell. I'll ask Elminster personally about it. e.g.: <code>/spell fireball</code>

/item (or /i) - search for an item. I'll cast Legend Lore spell to know what it is. e.g.: <code>/item bag of holding</code>

My code is open like your brain to a Mind Flayer!
You can get it <a href=\"{PROJECT_URL}\">here</a> (code, not brain)
Suggestions and contributions are welcome.")
}

pub fn help_roll_message() -> &'static str {
    r#"Consider an example:
<pre>
/roll 1d20 + 5 longsword
</pre>
<code>1</code> is a number of dices to throw. It can be any number OR <code>+</code> OR <code>-</code> to indicate advantage or disadvantage respectfully.
<code>d20</code> is, as you may guess, a dice to throw. <code>20</code> indicates how many faces it has, can be any number.
<code>5</code> is a "plain value", but it also could be another die, e.g. <code>1d4</code>
<code>longsword</code> is a comment for the roll.

Any part of this expression can be omitted, so all of these are valid expressions:
<code>/r</code> → 1d20
<code>/r 2</code> → 2d20
<code>/r +</code> → +d20 (d20 with advantage)
<code>/r -</code> → -d20 (d20 with disadvantage
<code>/r d4</code> → 1d4
<code>/r +5</code> → 1d20+5
<code>/r sneak</code> → 1d20 with "sneak" comment

You can also combine multiple rolls in one command:
<pre>
/r +5 longsword 3d6 piercing
</pre>"#
}
