use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Mutex;
use std::{borrow::Cow, cmp::min, convert::identity, mem, time::Instant};

use futures::StreamExt;
use hyper::Client;
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use regex::{Captures, Regex};
use simsearch::SimSearch;
use telegram_bot::{
    connector::{default_connector, hyper::HyperConnector, Connector},
    prelude::*,
    reply_markup,
    types::reply_markup::*,
    Api, CallbackQuery, GetMe, Message, MessageEntity, MessageEntityKind, MessageKind,
    MessageOrChannelPost, ParseMode, UpdateKind, User,
};
use thiserror::Error;

use crate::collection::{Collection, COMMANDS};
use crate::db::DndDatabase;
use crate::format::*;
use crate::metrics::{ERROR_COUNTER, MESSAGE_COUNTER, REQUEST_HISTOGRAM};
use crate::PROJECT_URL;

lazy_static! {
    static ref INITIAL_MARKUP: ReplyKeyboardMarkup = reply_markup!(
        reply_keyboard,
        resize,
        ["/roll", "/monster"],
        ["/spell", "/item"]
    );
}

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Telegram Error")]
    TelegramError(#[from] telegram_bot::Error),

    #[error("Database Error")]
    DbError(#[from] ejdb::Error),

    #[error("Format Error")]
    FormatError(#[from] FormatError),
}

pub struct Bot {
    db: DndDatabase,
    api: Api,
    me: User,
    cache: Mutex<HashMap<String, SimSearch<String>>>,
}

impl Bot {
    pub async fn new(db: DndDatabase) -> Result<Self, BotError> {
        let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
            error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
            std::process::exit(1)
        });

        let cache = Mutex::new(db.get_cache());
        let connector = get_connector();
        let api = Api::with_connector(token, connector);
        let me = api.send(GetMe).await?;

        Ok(Self { db, api, me, cache })
    }

    pub async fn start(self) {
        info!("Started successfully");
        let mut stream = self.api.stream();

        while let Some(Ok(update)) = stream.next().await {
            trace!("Received message: {:#?}", update);

            let result = match &update.kind {
                UpdateKind::Message(msg) => self.process_message(msg).await,
                UpdateKind::EditedMessage(_) => Ok(()),
                UpdateKind::ChannelPost(_) => Ok(()),
                UpdateKind::EditedChannelPost(_) => Ok(()),
                UpdateKind::InlineQuery(_) => Ok(()),
                UpdateKind::CallbackQuery(msg) => self.process_callback_query(msg).await,
                UpdateKind::Error(_) => Ok(()),
                UpdateKind::Unknown => Ok(()),
            };

            if let Err(err) = result {
                error!(
                    "Error occurred while processing message: {:?}, {:?}",
                    update, err
                );
            }
        }
    }

    async fn process_callback_query(&self, callback_query: &CallbackQuery) -> Result<(), BotError> {
        let callback_query = callback_query.clone();
        if let (Some(data), Some(msg)) = (callback_query.data, callback_query.message) {
            let (cmd, arg) = if let Some(sep_i) = data.find(" ") {
                if sep_i < data.len() - 1 {
                    (&data[0..sep_i], &data[sep_i + 1..data.len()])
                } else {
                    (data.as_ref(), "")
                }
            } else {
                (data.as_ref(), "")
            };

            trace!("Callback received: cmd=\"{}\", arg=\"{}\"", cmd, arg);

            self.execute_command(cmd, arg, &msg).await.map(|_| ())
        } else {
            Ok(())
        }
    }

    async fn process_message(&self, message: &Message) -> Result<(), BotError> {
        // We don't want to speak to other bots
        if message.from.is_bot {
            info!("Message from bot received: {:?}", message.kind);
            self.help(&message, "").await?;
            return Ok(());
        }

        let start_processing = Instant::now();
        match &message.kind {
            MessageKind::Text { data, entities } => {
                // No command was specified, but maybe it is a response to the previous command
                if entities.is_empty() {
                    if let Some(MessageOrChannelPost::Message(reply)) =
                        message.reply_to_message.clone().map(|reply| *reply)
                    {
                        if let MessageKind::Text {
                            data: reply_data,
                            entities: _,
                        } = reply.kind
                        {
                            // reply_data contains our own message generated in `search_item` function, e.g.: "What item should I look for? ..."
                            // The second word is our collection name
                            let mut iter = reply_data.split_whitespace();
                            let _ = iter.next();

                            if let Some(collection) = iter.next() {
                                self.execute_command(collection, data, &message).await?;
                            }
                        }
                    }
                    return Ok(());
                }

                let mut entities_iter = entities.into_iter().peekable();

                while let Some(entity) = entities_iter.next() {
                    match entity.kind {
                        MessageEntityKind::BotCommand => {
                            let (cmd, arg) =
                                self.parse_command(data, &entity, entities_iter.peek());

                            let timer = REQUEST_HISTOGRAM
                                .with_label_values(&[cmd.as_ref()])
                                .start_timer();

                            let cmd_result = self.execute_command(&cmd, &arg, &message).await;

                            timer.observe_duration();

                            let user_id: i64 = message.from.id.into();
                            let chat_type = chat_type_to_string(&message.chat);
                            let request = message.text().unwrap_or_default();

                            MESSAGE_COUNTER
                                .with_label_values(&[
                                    format!("{}", user_id).as_str(),
                                    chat_type_to_string(&message.chat),
                                    cmd.as_ref(),
                                ])
                                .inc();

                            let cmd_result = cmd_result.map_err(|err| err.into());

                            self.db.log_message(
                                user_id,
                                chat_type,
                                request,
                                &cmd_result,
                                Instant::now()
                                    .checked_duration_since(start_processing)
                                    .unwrap()
                                    .as_millis() as u64,
                            );

                            if let Err(err) = cmd_result {
                                ERROR_COUNTER.with_label_values(&[cmd.as_ref()]).inc();
                                error!("Error while processing message {}: {:#?}", data, err);
                                self.report_error(
                                    message.clone(),
                                    cmd.clone().to_owned(),
                                    data.clone(),
                                    err,
                                )
                                .await?;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    async fn execute_command(
        &self,
        cmd: &str,
        arg: &str,
        message: &Message,
    ) -> Result<Option<String>, BotError> {
        match cmd.as_ref() {
            // WARNING: ParseMode::Markdown doesn't work for some reason on large text with plain-text url
            // The returned string value is used to log request-response pair into the database
            "help" | "h" | "about" | "start" => self.help(message, arg).await,
            "roll" | "r" => self.roll(message, arg).await,
            "stats" => self.stats(message).await,
            _ => {
                if let Some(item) = COMMANDS.get(cmd) {
                    self.search_item(item, message, arg).await
                } else {
                    self.unknown(message, cmd).await
                }
            }
        }
    }

    async fn report_error(
        &self,
        message: Message,
        cmd: String,
        data: String,
        err: Box<dyn Error>,
    ) -> Result<Option<String>, telegram_bot::Error> {
        let url = format!("{}/-/issues/new?issue[title]=Error while processing command {}&issue[description]={}\n{}", PROJECT_URL, cmd, data, err);
        let msg = format!(
            "Oops! An error occurred :(\nPlease, [submit a bug report]({})",
            url
        );
        // TODO: Send email automatically to GitLab Service Desk
        self.api
            .send(
                message
                    .chat
                    .text(msg)
                    .parse_mode(ParseMode::Markdown)
                    .disable_preview(),
            )
            .await?;
        Ok(None)
    }

    async fn unknown(&self, message: &Message, cmd: &str) -> Result<Option<String>, BotError> {
        self.api
            .send(
                message
                    .chat
                    .text(format!("Err, I don't know `{}` command yet.", cmd))
                    .parse_mode(ParseMode::Markdown),
            )
            .await?;
        Ok(None)
    }

    async fn help(&self, message: &Message, _arg: &str) -> Result<Option<String>, BotError> {
        lazy_static! {
            static ref HELP_MARKUP: InlineKeyboardMarkup = reply_markup!(inline_keyboard,
                ["Source Code" url PROJECT_URL, "Buy me a coffee" url "https://paypal.me/bemyak", "Chat with author" url "https://t.me/bemyak"]
            );
        }
        let help = help_message();

        self.api
            .send(
                message
                    .chat
                    .text(help)
                    .parse_mode(ParseMode::Markdown)
                    .disable_preview()
                    .reply_markup(HELP_MARKUP.clone()),
            )
            .await?;

        Ok(None)
    }

    async fn roll(&self, message: &Message, arg: &str) -> Result<Option<String>, BotError> {
        let response = match roll_dice(arg) {
            Ok(response) => response,
            Err(err) => match err {
                FormatError::Other(_) => return Err(BotError::FormatError(err)),
                _ => err.to_string(),
            },
        };
        self.split_and_send(message, &response, None, vec!['\n'])
            .await?;
        Ok(Some(response))
    }

    async fn stats(&self, message: &Message) -> Result<Option<String>, BotError> {
        let last_update = Instant::now()
            .checked_duration_since(self.db.get_update_timestamp())
            .unwrap()
            .as_secs();

        let update_str = match last_update {
            0..=60 => format!("{}s", last_update),
            61..=3600 => format!("{}m", last_update / 60),
            3601..=86400 => format!("{}h", last_update / 60 / 60),
            86401..=std::u64::MAX => format!("{}d", last_update / 60 / 60 / 24),
        };

        let collection_metadata = self.db.get_metadata()?;
        let messages = self.db.get_all_massages()?;

        let msg = format!(
            "*Table stats*\n{}\n\n*Usage stats* (since last month / total)\n{}\n\nLast database update `{}` ago",
            format_collection_metadata(collection_metadata),
            format_message_stats(messages)?,
            update_str,
        );

        self.api
            .send(
                message
                    .chat
                    .text(msg.clone())
                    .parse_mode(ParseMode::Markdown),
            )
            .await?;
        Ok(Some(msg))
    }

    async fn search_item(
        &self,
        lookup_item: &Collection,
        message: &Message,
        arg: &str,
    ) -> Result<Option<String>, BotError> {
        if arg.is_empty() {
            let mut force_reply = ForceReply::new();
            force_reply.selective();

            self.api
                .send(
                    message
                        .chat
                        .text(format!(
                            "What {} should I look for? Please, reply with a name:",
                            lookup_item.get_default_collection()
                        ))
                        .reply_markup(force_reply),
                )
                .await?;

            return Ok(None);
        }

        let exact_match_result = lookup_item
            .collections
            .iter()
            .map(|collection| self.db.get_item(collection, arg))
            .filter_map(Result::ok)
            .filter_map(identity)
            .next();

        match exact_match_result {
            Some(item) => {
                let mut msg = format_document(item);
                let mut keyboard = InlineKeyboardMarkup::new();
                replace_links(&mut msg, &mut keyboard);
                self.split_and_send(
                    message,
                    &msg,
                    Some(ReplyMarkup::InlineKeyboardMarkup(keyboard)),
                    vec![' '],
                )
                .await?;
                Ok(Some(msg))
            }
            None => {
                let mut keyboard = InlineKeyboardMarkup::new();
                let mut found = false;

                let map = self.cache.try_lock().unwrap();

                for collection in lookup_item.collections.clone() {
                    let engine = map.get(collection).unwrap();
                    let results = engine.search(arg);

                    results.iter().for_each(|item| {
                        let command = format!("{} {}", lookup_item.get_default_collection(), item);
                        let button = InlineKeyboardButton::callback(item.clone(), command);

                        found = true;
                        keyboard.add_row(vec![button]);
                    });
                }

                let mut msg = if found {
                    format!(
                        "I don't have any {} with this exact name, but these looks similar:",
                        lookup_item.get_default_collection()
                    )
                } else {
                    format!(
                        "Can't find any {} with this name, sorry :(",
                        lookup_item.get_default_collection()
                    )
                };

                replace_links(&mut msg, &mut keyboard);
                self.split_and_send(
                    message,
                    &msg,
                    Some(ReplyMarkup::InlineKeyboardMarkup(keyboard)),
                    vec![' '],
                )
                .await?;

                Ok(Some(msg.to_owned()))
            }
        }
    }

    async fn split_and_send(
        &self,
        msg: &Message,
        text: &str,
        keyboard: Option<ReplyMarkup>,
        separators: Vec<char>,
    ) -> Result<(), telegram_bot::Error> {
        if text.is_empty() {
            return Ok(());
        }

        let messages = split(text, separators);
        let (last, all) = messages.split_last().unwrap();

        for text in all {
            self.api
                .send(msg.chat.text(*text).parse_mode(ParseMode::Markdown))
                .await?;
        }

        let mut answer = msg.chat.text(*last);

        answer.parse_mode(ParseMode::Markdown);

        if let Some(markup) = keyboard {
            answer.reply_markup(markup);
        }

        self.api.send(answer).await?;
        Ok(())
    }

    fn parse_command(
        &self,
        data: &String,
        entity: &MessageEntity,
        next_entity: Option<&&MessageEntity>,
    ) -> (String, String) {
        // We need to cut off the leading "/"
        let cmd_start = entity.offset as usize + 1;
        let cmd_end = (entity.offset + entity.length) as usize;

        // In group chats command might be provided in /command@bot_name format
        // So, we need to check if it is us who were asked
        let mut name_start = cmd_end;
        if let Some(i) = data.rfind('@') {
            if i + 1 < name_start {
                let bot_name = data[i + 1..name_start].to_owned();
                if Some(bot_name) == self.me.username {
                    name_start = i
                }
            }
        }

        let cmd = &data[cmd_start..name_start];

        let arg_start = cmd_end;
        let arg_end = next_entity.map_or(data.len(), |next_entity| next_entity.offset as usize);
        let arg = data[arg_start..arg_end].trim();

        // If there is no args, it could be that they are specified as part of the command
        // e.g.: /roll_2d8@bot_name
        if arg.is_empty() {
            let decoded_cmd = tg_decode(cmd);
            trace!("{}", decoded_cmd);
            let mut iter = decoded_cmd.split("_");
            (
                iter.next().unwrap_or(cmd).to_owned(),
                iter.collect::<Vec<&str>>().join(" "),
            )
        } else {
            (cmd.to_owned(), arg.to_owned())
        }
    }
}

pub fn get_connector() -> Box<dyn Connector> {
    env::var("roll_bot_http_proxy")
        .or(env::var("ROLL_BOT_HTTP_PROXY"))
        .or(env::var("http_proxy"))
        .or(env::var("HTTP_PROXY"))
        .or(env::var("https_proxy"))
        .or(env::var("HTTPS_PROXY"))
        .map_err(|err| Into::<Box<dyn Error>>::into(err))
        .and_then(|proxy_url| {
            info!("Running with proxy: {}", proxy_url);
            let connector = HttpsConnector::new();
            let proxy = Proxy::new(Intercept::All, proxy_url.parse()?);
            let connector = ProxyConnector::from_proxy(connector, proxy)?;
            let connector: Box<dyn Connector> =
                Box::new(HyperConnector::new(Client::builder().build(connector)));
            Ok(connector)
        })
        .unwrap_or(default_connector())
}

pub fn replace_links<'a>(text: &'a mut String, keyboard: &mut InlineKeyboardMarkup) {
    lazy_static! {
        static ref LINK_REGEX: Regex =
            Regex::new(r"\{@(?P<cmd>\w+)(?: (?P<arg>.+?)(?:\|(?P<source>\w+))?)?\}(?P<bonus>\d+)?")
                .unwrap();
    }

    let text_copy = text.clone();

    let result: Cow<str> = LINK_REGEX.replace_all(&text_copy, |caps: &Captures| {
        let cmd = caps.name("cmd").unwrap().as_str();
        let arg = caps.name("arg").map(|cap| cap.as_str()).unwrap_or_default();
        let source = caps
            .name("source")
            .map(|cap| cap.as_str())
            .unwrap_or_default();

        match cmd {
            "h" => match caps.name("bonus") {
                Some(bonus) => {
                    let bonus = bonus.as_str();
                    let roll_results = roll_results(&format!("d20+{}", bonus)).unwrap();
                    let roll = roll_results.get(0).unwrap();
                    format!("+{} `[{}]`", bonus, roll.total)
                }
                None => "+".to_owned(),
            },
            "atk" => "".to_owned(),
            "scaledamage" => source.to_owned(),
            "dice" | "damage" => {
                let roll_results = roll_results(arg).unwrap();
                let roll = roll_results.get(0).unwrap();
                format!("{} `[{}]` ", arg, roll.total)
            }
            _ => {
                if let Some(item) = COMMANDS.get(cmd) {
                    keyboard.add_row(vec![InlineKeyboardButton::callback(
                        format!("{}: {}", item.get_default_command(), arg),
                        format!("{} {}", item.get_default_command(), arg),
                    )]);
                    format!("_{}_ ", arg)
                } else {
                    format!("_{}_ ", arg)
                }
            }
        }
    });

    mem::replace(text, result.into_owned());
}

fn split(text: &str, separators: Vec<char>) -> Vec<&str> {
    const MAX_TEXT_LEN: usize = 4096;
    let mut result = Vec::new();

    let bytes = text.as_bytes();

    let mut start = 0;
    let mut end;

    while start < bytes.len() {
        let hard_end = min(start + MAX_TEXT_LEN, bytes.len());
        end = {
            if hard_end == bytes.len() {
                hard_end
            } else {
                bytes[start..hard_end]
                    .iter()
                    .rposition(|c| separators.contains(&(*c as char)))
                    .map(|i| i + start)
                    .unwrap_or(hard_end)
            }
        };

        // We already know that it is a valid utf8
        let s = unsafe { std::str::from_utf8_unchecked(&bytes[start..end]) };
        result.push(s);
        start = end + 1;
    }

    result
}
