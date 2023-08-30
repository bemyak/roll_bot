use std::{borrow::Cow, env, time::Instant, vec};

use ejdb::bson_crate::{ordered::OrderedDocument, Bson};
use inflector::Inflector;
use itertools::Itertools;
use regex::{Captures, Regex};
use reqwest::Url;
use thiserror::Error;

use teloxide::{
	adaptors::{throttle::Limits, CacheMe, Throttle},
	prelude::*,
	types::{
		ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, MessageId, MessageKind, ParseMode,
		ReplyMarkup, Update,
	},
	utils::command::{BotCommands, ParseError},
	RequestError,
};

use crate::{
	collection::{Collection, COMMANDS},
	commands::{HelpOptions, RollBotCommands},
	format::{
		self,
		db::{format_collection_metadata, format_message_stats},
		item::Item,
		monster::Monster,
		roll::{roll_results, DieFormatError},
		spell::Spell,
		telegram::chat_type_to_string,
		utils::HtmlEscapable,
	},
	DB, DONATION_URL, PROJECT_URL,
};

type RollBot = Throttle<CacheMe<Bot>>;

pub async fn start() {
	let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
		error!("You must provide <code>ROLL_BOT_TOKEN</code> environment variable!");
		std::process::exit(1)
	});

	let bot = Bot::new(token).cache_me().throttle(Limits::default());

	bot.set_my_commands(RollBotCommands::bot_commands())
		.await
		.log_on_error()
		.await;

	let handler = dptree::entry()
		.branch(
			Update::filter_message()
				.branch(
					dptree::entry()
						.filter_command::<RollBotCommands>()
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
		.enable_ctrlc_handler()
		.build()
		.dispatch()
		.await;
}

#[derive(Error, Debug)]
pub enum BotError {
	#[error("Request Error {0}")]
	Request(#[from] RequestError),

	#[error("Database Error {0}")]
	Db(#[from] ejdb::Error),

	#[error("Die Format Error {0}")]
	DieFormat(#[from] DieFormatError),

	#[error("Parse Error {0}")]
	Parse(#[from] ParseError),

	#[error("Entry Format Error {0}")]
	EntryFormat(String),

	#[error("Bad reply {0}")]
	BadReply(String),

	#[error("No reply text was produced {0}")]
	NoReplyText(String),

	#[error("Bad callback")]
	BadCallback,
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

async fn process_command(msg: Message, bot: RollBot, cmd: RollBotCommands) -> Result<(), BotError> {
	let start_processing = Instant::now();
	let chat_id = msg.chat.id;
	let chat_kind = msg.chat.kind.clone();
	let msg_text = msg.text().unwrap_or_default().to_string();

	if msg.via_bot != bot.get_me().await.ok().map(|bot| bot.user) {
		trace!(
			"Got message from @{}: {}",
			msg.from()
				.map(|user| user.username.as_ref().unwrap_or(&user.first_name).as_str())
				.unwrap_or("unknown"),
			msg.text().unwrap_or_default()
		);
	}

	let response = match cmd {
		RollBotCommands::Help(opts) => print_help(msg, bot, opts).await,
		RollBotCommands::Roll(roll) => {
			let reply_markup = msg.reply_markup().cloned().unwrap_or_else(|| {
				InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
					"Reroll", "reroll",
				)]])
			});

			let reply_id = msg.id;

			let roll = format!(
				"<b>{} rolls:</b>\n{}",
				msg.from().unwrap().first_name.escape_html(),
				roll
			);

			split_and_send(
				msg,
				bot,
				&roll,
				Some(ReplyMarkup::InlineKeyboard(reply_markup)),
				Some(reply_id),
			)
			.await
		}
		RollBotCommands::Stats => {
			let mut m = bot
				.send_message(msg.chat.id, stats()?)
				.parse_mode(ParseMode::Html)
				.disable_web_page_preview(true);
			if let Some(thread_id) = msg.thread_id {
				m = m.message_thread_id(thread_id);
			}
			m.await.map_err(BotError::Request)
		}
		RollBotCommands::Query((collection, item)) => {
			search_item(msg, bot, collection, &item).await
		}
		RollBotCommands::Echo(err) | RollBotCommands::Error(err) => {
			let mut m = bot
				.send_message(msg.chat.id, err)
				.reply_to_message_id(msg.id)
				.parse_mode(ParseMode::Html)
				.disable_web_page_preview(true);
			if let Some(thread_id) = msg.thread_id {
				m = m.message_thread_id(thread_id);
			}
			m.await.map_err(BotError::Request)
		}
	}
	.map(|r| r.text().map(|s| s.to_owned()));

	DB.log_message(
		chat_id.0,
		chat_type_to_string(&chat_kind),
		msg_text,
		&response,
		Instant::now()
			.checked_duration_since(start_processing)
			.unwrap()
			.as_millis() as u64,
	);

	if let Err(err) = response {
		error!("Error when sending the message: {err}");
	}

	Ok(())
}

async fn process_callback_query(callback_msg: CallbackQuery, bot: RollBot) -> Result<(), BotError> {
	trace!(
		"Got callback from @{}: {:?}",
		callback_msg
			.from
			.username
			.as_ref()
			.unwrap_or(&callback_msg.from.first_name),
		callback_msg.data
	);

	let (Some(mut data), Some(mut msg)) = (callback_msg.data, callback_msg.message) else {
		return Err(BotError::BadCallback);
	};

	// Reroll special message
	if let MessageKind::Common(ref mut common_msg) = msg.kind {
		// common_msg.from = Some(callback_msg.from);

		if data == "reroll" {
			let Some(mut reply) = std::mem::take(&mut common_msg.reply_to_message) else {
				return Err(BotError::BadCallback);
			};
			if let MessageKind::Common(common_reply) = &mut reply.kind {
				common_reply.from = Some(callback_msg.from);
			}
			data = reply.text().ok_or(BotError::BadCallback)?.to_owned();
			msg = *reply;
		}
	}

	// Support for old buttons, remove after a while
	let data = if data.starts_with('/') {
		data
	} else {
		format!("/{data}")
	};

	let bot_user = bot
		.get_me()
		.await
		.expect("Should always be successful")
		.user;
	let cmd = RollBotCommands::parse(&data, bot_user.username.as_ref().unwrap())?;

	msg.via_bot = Some(bot_user);
	process_command(msg, bot, cmd).await
}

async fn print_help(msg: Message, bot: RollBot, opts: HelpOptions) -> Result<Message, BotError> {
	match opts {
		HelpOptions::None => {
			let kb = InlineKeyboardMarkup::new(vec![
				vec![
					InlineKeyboardButton::url("Source Code", Url::parse(PROJECT_URL).unwrap()),
					InlineKeyboardButton::url("Buy me a coffee", Url::parse(DONATION_URL).unwrap()),
				],
				vec![
					InlineKeyboardButton::url(
						"News",
						Url::parse("https://t.me/roll_bot_news").unwrap(),
					),
					InlineKeyboardButton::url(
						"Chat",
						Url::parse("https://t.me/roll_bot_chat").unwrap(),
					),
				],
			]);
			let mut m = bot
				.send_message(msg.chat.id, format::telegram::help_message())
				.parse_mode(ParseMode::Html)
				.reply_markup(ReplyMarkup::InlineKeyboard(kb))
				.disable_web_page_preview(true);
			if let Some(thread_id) = msg.thread_id {
				m = m.message_thread_id(thread_id);
			}
			m.await.map_err(BotError::Request)
		}
		HelpOptions::Roll => {
			let mut m = bot
				.send_message(msg.chat.id, format::telegram::help_roll_message())
				.parse_mode(ParseMode::Html)
				.disable_web_page_preview(true);
			if let Some(thread_id) = msg.thread_id {
				m = m.message_thread_id(thread_id);
			}
			m.await.map_err(BotError::Request)
		}
	}
}

fn stats() -> Result<String, BotError> {
	let last_update = Instant::now()
		.checked_duration_since(DB.get_update_timestamp())
		.unwrap()
		.as_secs();

	let update_str = match last_update {
		0..=60 => format!("{last_update}s"),
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

		let mut m = bot
			.send_message(
				msg.chat.id,
				format!(
					"What {} should I look for? Please, <b>reply</b> to this message with a name:",
					lookup_item.get_default_command()
				),
			)
			.parse_mode(ParseMode::Html)
			.disable_web_page_preview(true)
			.reply_markup(force_reply);

		if let Some(thread_id) = msg.thread_id {
			m = m.message_thread_id(thread_id);
		}

		return m.await.map_err(BotError::Request);
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
			// Deduplicate and sort keys
			keyboard.inline_keyboard = keyboard
				.inline_keyboard
				.into_iter()
				.unique()
				.sorted_by(|row1, row2| row1[0].text.cmp(&row2[0].text))
				.collect();
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
				None,
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
					"I've found these {} names:",
					lookup_item.get_default_command()
				)
			};

			replace_string_links(&mut reply_msg, &mut keyboard);
			split_and_send(
				msg,
				bot,
				&reply_msg,
				Some(ReplyMarkup::InlineKeyboard(keyboard)),
				None,
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
		let name = caps
			.name("arg1")
			.map(|cap| cap.as_str())
			.unwrap_or_default();
		let source = caps.name("arg2").map(|cap| cap.as_str());
		let display_text = caps.name("arg3").map(|cap| cap.as_str());
		let other = caps.name("arg4").map(|cap| cap.as_str());

		let nice_str = other.or(display_text).unwrap_or(name);

		let source = source.or(match cmd {
			"spell" => Some("PHB"),
			"item" => Some("DMG"),
			"class" => Some("PHB"),
			"subclass" => Some("PHB"),
			"creature" => Some("MM"),
			"condition" => Some("PHB"),
			"disease" => Some("DMG"),
			"status" => Some("DMG"),
			"background" => Some("PHB"),
			"race" => Some("PHB"),
			"optfeature" => Some("PHB"),
			"reward" => Some("DMG"),
			"feat" => Some("PHB"),
			"psionic" => Some("UATheMysticClass"),
			"object" => Some("DMG"),
			"cult" => Some("MTF"),
			"boon" => Some("MTF"),
			"trap" => Some("DMG"),
			"hazard" => Some("DMG"),
			"deity" => Some("PHB"),
			"variantrule" => Some("DMG"),
			"vehicle" => Some("GoS"),
			"vehupgrade" => Some("GoS"),
			"action" => Some("PHB"),
			"classFeature" => Some("PHB"),
			"subclassFeature" => Some("PHB"),
			"table" => Some("DMG"),
			"language" => Some("PHB"),
			"charoption" => Some("MOT"),
			"recipe" => Some("HF"),
			"itemEntry" => Some("DMG"),
			"quickref" => Some("PHB"),
			"skill" => Some("PHB"),
			"sense" => Some("PHB"),
			_ => None,
		});

		match cmd {
			"i" => format!("• {nice_str}"),
			"hit" => format!("+{name}"),
			"h" => match caps.name("bonus") {
				Some(bonus) => format!("<b>{}</b>", bonus.as_str()),
				None => "".to_owned(),
			},
			"atk" => "".to_owned(),
			"scaledamage" => format!("<b>{nice_str}</b>"),
			"dice" | "damage" => {
				let roll_results = roll_results(name).unwrap();
				let roll = roll_results.get(0).unwrap();
				format!("<b>{}</b> <code>[{}]</code> ", name, roll.expression.calc())
			}
			"recharge" => {
				if name.is_empty() {
					"(Recharge 6)".to_string()
				} else {
					format!("(Recharge {name}-6)")
				}
			}
			_ => {
				let nice_str = other.or(display_text).unwrap_or(name);
				if let Some(item) = COMMANDS.get(cmd) {
					let mut kb = keyboard.clone();
					kb = kb.append_row(vec![InlineKeyboardButton::callback(
						format!("{}: {}", item.get_default_command(), nice_str),
						format!(
							"{} {}{}",
							item.get_default_command(),
							name,
							source
								.map(|source| format!(" ({source})"))
								.unwrap_or_default()
						),
					)]);
					*keyboard = kb;
				}
				format!("<i>{nice_str}</i>")
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
	reply_msg_id: Option<MessageId>,
) -> Result<Message, BotError> {
	if text.is_empty() {
		return Err(BotError::NoReplyText(
			msg.text().unwrap_or_default().to_owned(),
		));
	}

	let messages = split2(text, 4096);
	let (last, all) = messages.split_last().unwrap();

	for text in all {
		let mut m = bot
			.send_message(msg.chat.id, text)
			.parse_mode(ParseMode::Html)
			.disable_web_page_preview(true);
		if let Some(thread_id) = msg.thread_id {
			m = m.message_thread_id(thread_id);
		}
		m.await?;
	}

	let mut answer = bot
		.send_message(msg.chat.id, last)
		.parse_mode(ParseMode::Html)
		.disable_web_page_preview(true);

	if let Some(markup) = keyboard {
		answer = answer.reply_markup(markup);
	}

	if let Some(reply_msg_id) = reply_msg_id {
		answer = answer.reply_to_message_id(reply_msg_id);
	}

	if let Some(thread_id) = msg.thread_id {
		answer = answer.message_thread_id(thread_id);
	}

	answer.await.map_err(BotError::Request)
}

fn get_closing_tag_len(v: &[String]) -> usize {
	v.iter().map(|s| s.len() + 3).sum()
}

fn split2(text: &str, max_len: usize) -> Vec<String> {
	enum State {
		Text,
		OpenTag,
		CloseTag,
	}

	let mut result = Vec::new();
	let mut part = String::new();
	let mut state = State::Text;
	let mut tags: Vec<String> = Vec::new();
	let mut tag: String = String::new();

	// In priority highest => lowest
	const SEPARATORS: [char; 2] = ['\n', ' '];
	// len = SEPARATORS.len() + 1
	// The last element is char split
	let mut sep_poss = [0, 0, 0];
	let mut sep_tags = [0, 0, 0];

	for c in text.chars() {
		if part.len() >= max_len {
			let (sep_pos, sep_tag_len) = sep_poss
				.iter()
				.enumerate()
				.find(|(_, &sep_pos)| sep_pos != 0)
				.map(|(sep_i, sep_pos)| (*sep_pos, sep_tags[sep_i]))
				.unwrap_or((part.len(), tags.len()));

			let mut new_part: String = part.drain(..sep_pos).collect();
			for tag in tags[..sep_tag_len].iter().rev() {
				new_part.push_str(&format!("</{tag}>"));
			}
			result.push(new_part);
			sep_poss = [0, 0, 0];

			if !part.is_empty() {
				part.remove(0);
			}
			let mut closing_tags = tags[..sep_tag_len]
				.iter()
				.map(|t| format!("<{t}>"))
				.collect::<Vec<_>>()
				.join("");
			closing_tags.push_str(&part);
			part = closing_tags;
		}

		match state {
			State::Text => match c {
				'<' => {
					state = State::OpenTag;
				}
				c => {
					if part.is_empty() && c.is_whitespace() {
						continue;
					}

					let sep = if let Some(sep_i) = SEPARATORS.iter().position(|&sep| sep == c) {
						Some((sep_i, part.len()))
					} else if c.is_ascii() {
						Some((SEPARATORS.len(), part.len() + 1))
					} else {
						None
					};

					if let Some((sep_i, sep_pos)) = sep {
						if sep_pos + get_closing_tag_len(&tags) <= max_len {
							sep_tags[sep_i] = tags.len();
							sep_poss[sep_i] = sep_pos;
						}
					}
				}
			},
			State::OpenTag => match c {
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
					tag = String::new();
					state = State::Text;
				}
				_ => tag.push(c),
			},
			State::CloseTag => match c {
				'>' => {
					if Some(&tag) == tags.last() {
						tags.pop();
					} else {
						warn!("Unexpected closing tag: {tag} in\n{text}")
					}
					tag = String::new();
					state = State::Text;
				}
				_ => tag.push(c),
			},
		}
		part.push(c);
	}

	result.push(part);
	result
}

#[test]
fn test_split2_simple0() {
	let parts = split2("123", 3);
	assert_eq!("123", parts[0]);
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
fn test_split2_simple3() {
	let parts = split2("123\n456\n789", 3);
	assert_eq!("123", parts[0]);
	assert_eq!("456", parts[1]);
	assert_eq!("789", parts[2]);
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

#[test]
fn test_split2_tags3() {
	let msg = r#"<b>\u{ad} rolls:</b>
	<code>10d⛧</code> [<̠̤̞̤̠̙̤̝̥̗̖̿̿̍̑̅̅͒̐̄b̢̡̡̧̛̛̘̘̖̅͗̃̐̊̈́͊ͥͥͥ̔̊̓͋̉̄́̇̽̓̃͗͂̍͗̊̒̓̋͌ͣ̋̄͗͌͑̍͘͘>̗̗̄͋̆͒͒̈́͑͑̊̓̅͊͑̎̑̑̍̄͒1̴̧̢̧̨̛̤̥̟̖̗̘̥̘̠̤̝̈͗͂̆̐̑̇̍̐͒̑͂͑̄̎̀̀̕͘<̡̧̢̧̢̨̛̯̜̯̟̼̟̳̮̙͍̬̮̲̺̦̤̪͓̯̪͕̜̯̥̺̤̹͕̜̻̝̖̱̥̟̐ͣͣͥ̆͂̽̍̓̀́̑̎̊̎̔̈̍ͣ̉ͩ̅ͣ̉ͨ̑ͨ͗̽͐̏ͨ̿ͣͣ͐̄̒ͩ̊̃̀̕͘ͅͅ/̢̡̢̢̡̡̥̪͈̮͓̤̖͎̘̻̘͉̙̩͉̻̗̯͎͈̜̥͇͇̦̮̤̺̩̪̳̂͌͊̈͋̈͌͂̇̎͋͌̑̈́̆̑͊̎͗̈́͒͗͘̕̕ͅͅͅb̡̡̢̧̡̡̢̼̥͉̜͍͖̯͖͖͉̮̘͖̫̫̘̙̯̺̫̹̝̲̲͎̟̗̪̯̯̼̲̗̱̺̪̅ͤ̏̌ͧͣͮ͊̉ͯ̎ͫ̋̊͗ͨͦ̑̇̄̿ͥ̀ͨ̃ͧ̈́ͯ̊̐̈ͨͩ̾ͤͭ̽̔́̂̍ͫ̍̓̊ͮ̆̑̀ͅ>̛̛̱͈̺̦̙̯̩̫̳̯͇͉̥̪̜̖̱̙̪̻͇̺̞͈̩̬̱̭̦̭̲̇̇͗͗̎̄͂͂͒͂͑̊̑̊̅̇́̕,̡̛̯̠̩̹̻̞̟̞̻̙̰̼̦̼̹̬̯̲̹̟̘̦̰̫̩̘̜̮̅̊͒̊͗̐̈́̇̍̇̇͑͑͒͑̍̆́́̕̕ͅͅ9̡̡̧̦̯̬̞̯̤̠̺̥̘̮̹̩̜̱̲̯̤̳̥̬̭̝̥̘̎̄̎̀̀̕͘͘,̝̩̪̥̤̘̟̖̜̤̠̟̙̪̪̜̍̄̍̎̿8̛̛̫̮̦̬̖̥̥̤̞̺̫͈̬̪̮̘̘̙͎͓̱̩̪͓͈̗͓͍̦̼̯̖̞͒͂̑̅̂̐̅̄͊̑͒͒̍̅̊̿͒͂͋̐͒̊̎̈̃́͘̕̕ͅ,̡̧̨̛̘̘̪̳̙̱̩̺̯̱̜̪̺̰̘̳̮̪̥̯̜̫̩̤̲̐̍̍̄̆̑̐̐̅́̀̀̕͘̕1̧̧̢̡̛̗̗̐̂͋̽͐̍̔ͣ̋ͤͤ͂̀͐͒̌̋͒͂̎̓̂̑̆́ͣͤ͊̓ͥ̂̿͌̃͑̃́͘͘3̘̖̜̯̜̯̹̬̦̫̭̜̯̹̗̩̮̤̬̮̞̻̜̞̹̠̘̅̆͑̎̅̄̄̐̎͑̑̍̐̕ͅ,͔͖̖̳̞̟̩̭͕͉̳̳̘̖̳̺̖̳͔̬͕̯̭̤̝̜͖̺͉̭̫̲̺̭̠̥͓̆̊̇̎͒̿͑̄̐̿̑̄͑̊́́̕̕ͅͅ1̯̫̫̮̩̥̪̗̟̯̩̠̜̟̦̘̫̠̙̄ͧ̊̐͗ͣ͐ͤ́̃ͤ̒̎͂̽ͤ͌͂̉̇̑͌͒̊̌̆̿ͥ̂̈̅̑̿͑͑͑̇̽́̀́̀͘5͉̯̘̤̮̥̺̤̖̤͈̺̬̯͉̝̳̱̪̟̰̻̤̻̞̮̞̲̘̠̄̄̄̄̅̕̕ͅ,̪̹̮̳̠̗̯̱̟̹̤̖̤̩̟̤̝̖̘̟̱̘̘̭͗͊̎̍̓͒̆͒̄̑̎͑͗̑̄͊͊̈̓̄̆͋̍͋̕1̠̠̝̘̤̙̤̙̘̞̑͒͑̑̆̄̎͗̍͒̑͑̑7̛̛̙̘̗̥̝̙̞̤̠̤̞̿̇͒̈́͗̈̐̑̄͌͌̍͊̅͑̑͒̅̿̃̎̀,̮̳̝̬̞̹̗̫̖̬̤̮̹̟̗̘̭̤̙̜̗̤̲̙͂͑̍̍̇̓͒̐̓͗̐̄̇̐͂̎̀̕̕<̤̦̖̥̙̙̖̤̥̞̜̖̘͂̍̊̓̄̋̐͊̓͑̓̓̆ͥ̆́̈ͣ͊ͣ̀̿͑͗ͥ̀̆̃̅͐͌͊̏̓̑͑̕b̦̝̞̝̞̗̜̦̠̠̖̝̐̿̐͗͐̄͑̽̍́̐͑͗̑̄̊͂͑̿̓̒͌́̑̉̒͑̏̅̐̈̐̅̌̕>̞̖̖̥̞̠̠̞̞̤̝̅͊̈͋͌͐ͦ͋ͨ̿̐ͯ͑̈̂̋̒̊̈͗͐͊̍̌ͥͮͧ̿͒ͬ̓̌ͨ͗ͣ̏̀̾ͪ̍̈͋̋̇̐͋̈1̜̘̜̜̖̿̓̿̈́̊̍̐͊̑͒͋̎̇̓̐̅̅͊͂̍<̡̛̩̟̤̰̬̤̞̠̝̘̩̮̤̥̜̝̙̰̤̦͊͑̊̑̑͂̓͑̈̐̿̈̅̿̈́̄͋̇͌̅̈̀̀́͘̕/̞̟̗̟̞̞̖̜̌̿ͧ̈̐̓̈́ͪ̔̎̏̆̎ͦ͂̃ͤ̓̿̀̈̓ͪͪ͑͌ͣ͐̽͒ͧ͋̋͐̉ͥͪ̓̄̊ͪb̛̹̹̙̮̫̪̠̠̬̥̟̠̜̝̟̪̪̦̙̪̥̜̭̮̟̠͗͑̑̊̊̅̅̊͂̓̅͒̈͑̿̄̇̄̀̕>̴̢̨̡̛̟̘̠̙̟̖̗̦̩̟̖̥̝͋̍̀̊͊͋́̏͗̄̋̈̎͋̇̈͌͋͂̓̊͊̄̎͂̋́̍́́́̀́,̘̖̗̘̗̗̖̐̌̌̏̔͋̎̔̽̄͒̆̊͊̐̄͋̑̒̓̔́̓̍̿̀̐̊̏̀̀͒<̦̝̞̯̪̫̯̫̼̜̪̖̼̞̝̟̺̩̗̙̼̹̮̦̗̥̙̎̆͗̇̈͂̍̈́͋̆̃͋͒̎͌̇͋̿͗̿̈̕̕ͅb̴̧̡̨̧̛͍͈̯̰̩͇̭̻̼͖̝̲̪̪̹̭̻͙͖̫͎̬̥̗͙͉̭͙̼̻͎̰̮͇̭͂͐̑ͤ̅̂̅͌͐̄ͦͩ̿ͤ͐͌ͧ̈͐̿͋̀̂ͣ̂̒ͤ̎͌͊ͪͤ́̅͑͋̉̎̍́̓̀̀͘͘ͅͅͅ>̛̛̟̗̘̖̜̤̠̥̤̙̞̟ͤ̆ͤͩ̈́͗̅ͬͧ̓͊̒ͣ̉̃̒̓̎̏̿ͣ̽ͤ̀̐̈́ͤͣ͐͊͋̋̌͐͌̔̍ͥ̍ͦ͌͌̌1̛̛̝̤̥̠̩̦̗̝̘̞̗̖̩̝̽̓̓̿ͣ̋̽̍͗͋̑͌͊͐̋́̃̓̓͐̓͋͂̄̋͒̍̐͋̂̒̉́́͒́̕͘<̪̦̜̘̝̙̤̠̖̙̜̗̫̟̗̆͒͂͗̈́̄̅̇͗̎͌͒̈́̈͒̓̐̑̊͋̕/̛̦̘̜̙̫̠̜̥̪̝̪̝̟̠̝̅̑̄̆̍̅̊͒͑̑͒̆̈̅̕b̴̨̡̨̧̢̢̛̛̘̖̙̟̜̗̗̙̎̄̑̑̑̑́́>̡̧̖͒̄̑̀ͥͤ̒̈́̒̃ͬ̐̋ͨ̑͑ͣ̏͑͂̄͋̽ͬͭ̃ͫ̃̄̂ͩ͋ͮ̈̔ͫ̀ͥ̏ͥ̐ͥͤ͂̐́̀̀́̀̕,̡̧̛̦̥̫̝̤̪̠̜̤̠̞̩̠̦̠̓̍͋͂̓̂̒̋̐̉̅̅̔̿̂̎͋̅̀̊͑͌̀̋̌̏́̊̉̆̆̐́̆͑͑̀́́͘͘2̛̫̞̹͕͙͉̝̲͉̘̩̬͇̥̺̩͇͖̳̘̳̼͕̗̯̗̪̼̗̱̠̩͎͈̳̯̰͔̜̎̍̕,̛̫̩̫̝̟̙̩̘̙̞̫̗̘̫̠̥ͣ̐̒̌̄̌̉̑ͦ̓̋̈́̅ͣ̄̍͗͊̊͐̎ͩ͌̊̀̐̀̊ͤ̓̑̐͋ͣ̑̐ͣ͌͗̈̕1̴̢̡̨̢̧̤̦̭̥̥̭̝̝̙̗̫̹̪̭̭̺̜̰̟̜̞̹̙̞̬̌̿͊̓̿̉́̿̒́̄͊̌͗̍̆͐̿̄̽͂ͣ̽ͣ̎͗͌͒͑̇͌͗́̂̆̇̀́́̕2̘̝̘̖̘̙̙̎̎] = 79"#;
	let parts = split2(msg, 4096);
	assert_eq!("<b>\u{ad} rolls:</b>", parts[0]);
	assert_eq!(3, parts.len());
}
