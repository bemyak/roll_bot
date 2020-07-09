use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Mutex;
use std::{borrow::Cow, convert::identity, time::Instant};

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

use crate::db::DndDatabase;
use crate::format::*;
use crate::PROJECT_URL;
use crate::{ERROR_COUNTER, MESSAGE_COUNTER, REQUEST_HISTOGRAM};

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
                                self.execute_command(&format!("/{}", collection), data, &message)
                                    .await?;
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
            "/help" | "/h" | "/about" | "/start" => self.help(message, arg).await,
            "/roll" | "/r" => self.roll(message, arg).await,
            "/stats" => self.stats(message).await,
            "/item" | "/i" => {
                self.search_item(vec!["item", "baseitem"], message, arg)
                    .await
            }
            "/monster" | "/m" => self.search_item(vec!["monster"], message, arg).await,
            "/spell" | "/s" => self.search_item(vec!["spell"], message, arg).await,
            _ => self.unknown(message, cmd).await,
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
        let response = roll_dice(arg)?;
        self.api
            .send(
                message
                    .chat
                    .text(response.clone())
                    .parse_mode(ParseMode::Markdown),
            )
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
        collections: Vec<&str>,
        message: &Message,
        arg: &str,
    ) -> Result<Option<String>, BotError> {
        let default_collection_name = collections.get(0).clone().unwrap_or(&"item");

        if arg.is_empty() {
            let mut force_reply = ForceReply::new();
            force_reply.selective();

            self.api
                .send(
                    message
                        .chat
                        .text(format!(
                            "What {} should I look for? Please, reply with a name:",
                            default_collection_name
                        ))
                        .reply_markup(force_reply),
                )
                .await?;

            return Ok(None);
        }

        let exact_match_result = collections
            .iter()
            .map(|collection| self.db.get_item(collection, arg))
            .filter_map(Result::ok)
            .filter_map(identity)
            .next();

        match exact_match_result {
            Some(item) => {
                let msg = format_document(item);
                self.send_item(message, &msg, None).await?;
                Ok(Some(msg))
            }
            None => {
                let mut keyboard = InlineKeyboardMarkup::new();
                let mut found = false;

                let map = self.cache.try_lock().unwrap();

                for collection in collections.clone() {
                    let engine = map.get(collection).unwrap();
                    let results = engine.search(arg);

                    let command_prefix = "/".to_owned() + default_collection_name + " ";

                    results.iter().for_each(|item| {
                        let command = command_prefix.clone() + item;
                        let button = InlineKeyboardButton::callback(item.clone(), command);

                        found = true;
                        keyboard.add_row(vec![button]);
                    });
                }

                let msg = if found {
                    format!(
                        "I don't have any {} with this exact name, but these looks similar:",
                        default_collection_name
                    )
                } else {
                    format!(
                        "Can't find any {} with this name, sorry :(",
                        default_collection_name
                    )
                };

                self.send_item(message, &msg, Some(keyboard)).await?;

                Ok(Some(msg.to_owned()))
            }
        }
    }

    async fn send_item(
        &self,
        msg: &Message,
        text: &str,
        keyboard: Option<InlineKeyboardMarkup>,
    ) -> Result<(), telegram_bot::Error> {
        let mut keyboard = keyboard.unwrap_or(InlineKeyboardMarkup::new());
        let text = replace_links(text, &mut keyboard);

        let max_text_len = 4096;

        let mut start = 0;
        let mut end;

        while text.len() - start > max_text_len {
            end = text[start..start + max_text_len]
                .rfind(" ")
                .unwrap_or(start + 4096);
            self.api
                .send(
                    msg.chat
                        .text(&text[start..end])
                        .parse_mode(ParseMode::Markdown),
                )
                .await?;
            start = end + 1;
        }

        self.api
            .send(
                msg.chat
                    .text(&text[start..text.len()])
                    .parse_mode(ParseMode::Markdown)
                    .reply_markup(keyboard),
            )
            .await?;
        Ok(())
    }

    fn parse_command(
        &self,
        data: &String,
        entity: &MessageEntity,
        next_entity: Option<&&MessageEntity>,
    ) -> (String, String) {
        let cmd_start = entity.offset as usize;
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

fn replace_links<'a>(text: &'a str, keyboard: &mut InlineKeyboardMarkup) -> Cow<'a, str> {
    lazy_static! {
        static ref LINK_REGEX: Regex =
            Regex::new(r"\{@(?P<cmd>\w+)(?: (?P<arg>.+?)(?:\|(?P<source>\w+))?)?\}(?P<bonus>\d+)?")
                .unwrap();
    }

    LINK_REGEX.replace_all(text, |caps: &Captures| {
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
            "dice" | "damage" => {
                let roll_results = roll_results(arg).unwrap();
                let roll = roll_results.get(0).unwrap();
                format!("{} `[{}]` ", arg, roll.total)
            }
            "item" | "spell" | "monster" => {
                keyboard.add_row(vec![InlineKeyboardButton::callback(
                    arg,
                    format!("/{} {}", cmd, arg),
                )]);
                format!("_{}_ ", arg)
            }
            "scaledamage" => source.to_owned(),
            _ => format!("_{}_ ", arg),
        }
    })
}
