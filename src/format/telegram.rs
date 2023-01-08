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
	format!("Hi! I'm a bot. The Roll Bot!
I can help you with your Dungeons&Dragons game (5th edition) and many others that require throwing dices. I can:

/roll (or /r) - roll a die. By default I'll use d20, but you can give me any number of dices! e.g.: <code>/roll 2d6 +5</code>
run <code>/help roll</code> to learn all the secrets of this command

/monster (or /m) - search for a monster. I'll look in every book in Candlekeep and find at least one. e.g.: <code>/monster tarasque</code>

/spell (or /s) - search for a spell. I'll ask Elminster personally about it. e.g.: <code>/spell fireball</code>

/item (or /i) - search for an item. I'll cast Legend Lore spell to know what it is. e.g.: <code>/item bag of holding</code>

My code is open like your brain to a Mind Flayer!
You can get it <a href=\"{PROJECT_URL}\">here</a> (code, not brain)
Suggestions and contributions are welcome.")
}

pub fn help_roll_message() -> &'static str {
	r#"The bot supports full <a href="https://en.wikipedia.org/wiki/Dice_notation">dice notation</a>.
Consider an example:
<pre>
/roll 1d20 + 5 longsword
</pre>
<code>1</code> is a number of dices to throw. It can be any number OR <code>+</code> OR <code>-</code> to indicate advantage or disadvantage respectfully.
<code>d20</code> is, as you may guess, a dice to throw. <code>20</code> indicates how many faces it has, can be any number.
<code>5</code> is a "plain value", but it also could be another die, e.g. <code>1d4</code>
<code>longsword</code> is a comment for the roll.

There many shortcuts to avoid wearing-out your keyboard:
<code>/r</code> → 1d20
<code>/r 2</code> → 2d20
<code>/r d4</code> → 1d4
<code>/r +</code> → +d20 (d20 with advantage)
<code>/r -</code> → -d20 (d20 with disadvantage
<code>/r +5</code> → 1d20+5
<code>/r 2d%</code> → 2d100

Fudge/Fate dices are supported:
<code>/r 4dF</code>

Also, dice selectors can be applied to any roll:
<code>5d20kh2</code> → <b>keep</b> 2 <b>highest</b> dices out of 5
<code>5d20kl2</code> → <b>keep</b> 2 <b>lowest</b> dices
<code>5d20dh2</code> → <b>drop</b> 2 <b>highest</b> dices
<code>5d20dl2</code> → <b>drop</b> 2 <b>lowest</b> dices

And a couple of shortcuts again:
<code>2d20H</code> → 2d20kh1 (a.k.a. roll with advantage)
<code>2d20L</code> → 2d20kl1 (a.k.a. roll with disadvantage)

Selector can be chained together:
<code>5d20kh2dh1</code> → gives the second best roll of 5 d20
"#
}
