use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::Instant;

use futures::StreamExt;
use hyper::Client;
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use telegram_bot::{
    connector::{default_connector, hyper::HyperConnector, Connector},
    prelude::*,
    reply_markup,
    types::reply_markup::*,
    Api, GetMe, Message, MessageEntityKind, MessageKind, ParseMode, UpdateKind, User,
};

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

pub struct Bot {
    db: DndDatabase,
    api: Api,
    cache: HashMap<String, Vec<String>>,
    cache_timestamp: Instant,
    me: User,
}

impl Bot {
    pub async fn new(db: DndDatabase) -> Result<Self, Box<dyn Error>> {
        let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
            error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
            std::process::exit(1)
        });

        let cache = db.get_cache();

        let connector = get_connector()?;
        let api = Api::with_connector(token, connector);
        let me = api.send(GetMe).await?;

        Ok(Self {
            db,
            api,
            cache,
            cache_timestamp: Instant::now(),
            me,
        })
    }

    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        info!("Starting roll_bot...");
        let mut stream = self.api.stream();

        while let Some(Ok(update)) = stream.next().await {
            trace!("Received message: {:#?}", update);

            let result = match &update.kind {
                UpdateKind::Message(msg) => self.process_message(msg).await,
                UpdateKind::EditedMessage(_) => Ok(()),
                UpdateKind::ChannelPost(_) => Ok(()),
                UpdateKind::EditedChannelPost(_) => Ok(()),
                UpdateKind::InlineQuery(_) => Ok(()),
                UpdateKind::CallbackQuery(_) => Ok(()),
                UpdateKind::Error(_) => Ok(()),
                UpdateKind::Unknown => Ok(()),
            };

            if let Err(err) = result {
                error!(
                    "Error occurred while processing message: {:?}, {}",
                    update, err
                );
            }
        }

        Ok(())
    }

    async fn process_message(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        // We don't want to speak to other bots
        if message.from.is_bot {
            info!("Message from bot received: {:?}", message.kind);
            self.help(&message, "").await?;
            return Ok(());
        }

        let start_processing = Instant::now();
        match &message.kind {
            MessageKind::Text { data, entities } => {
                let mut entities_iter = entities.into_iter().peekable();

                while let Some(entity) = entities_iter.next() {
                    match entity.kind {
                        MessageEntityKind::BotCommand => {
                            let cmd_start = entity.offset as usize;
                            let cmd_end = (entity.offset + entity.length) as usize;

                            // In group chats command might be provided in /command@bot_name format
                            // So, we need to check if it is us who were asked
                            let mut name_start = cmd_end;
                            if let Some(i) = data.find('@') {
                                if i + 1 < name_start {
                                    let bot_name = data[i + 1..name_start].to_owned();
                                    if Some(bot_name) == self.me.username {
                                        name_start = i
                                    } else {
                                        trace!("Not me!");
                                        continue;
                                    }
                                }
                            }

                            let cmd = &data[cmd_start..name_start];

                            let timer = REQUEST_HISTOGRAM.with_label_values(&[cmd]).start_timer();

                            let arg_start = cmd_end;
                            let arg_end = entities_iter
                                .peek()
                                .map_or(data.len(), |next_entity| next_entity.offset as usize);
                            let arg = &data[arg_start..arg_end].trim();

                            let cmd_result = match cmd {
                                // WARNING: ParseMode::Markdown doesn't work for some reason on large text with plain-text url
                                // The returned string value is used to log request-response pair into the database
                                "/help" | "/h" | "/about" | "/start" => {
                                    self.help(&message, arg).await
                                }
                                "/roll" | "/r" => self.roll(&message, arg).await,
                                "/stats" => self.stats(&message).await,
                                _ => self.unknown(&message, cmd).await,
                            };

                            timer.observe_duration();

                            let user_id: i64 = message.from.id.into();
                            let chat_type = chat_type_to_string(&message.chat);
                            let request = message.text().unwrap_or_default();

                            MESSAGE_COUNTER
                                .with_label_values(&[
                                    format!("{}", user_id).as_str(),
                                    chat_type_to_string(&message.chat),
                                    cmd,
                                ])
                                .inc();

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
                                ERROR_COUNTER.with_label_values(&[cmd]).inc();
                                error!("Error while processing message {}: {}", data, err);
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

    async fn report_error(
        &self,
        message: Message,
        cmd: String,
        data: String,
        err: Box<dyn Error>,
    ) -> Result<Option<String>, Box<dyn Error>> {
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

    async fn unknown(
        &self,
        message: &Message,
        cmd: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
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

    async fn help(&self, message: &Message, _arg: &str) -> Result<Option<String>, Box<dyn Error>> {
        lazy_static! {
            static ref HELP_MARKUP: InlineKeyboardMarkup = reply_markup!(inline_keyboard,
                ["Source Code" url PROJECT_URL, "Buy me a coffee" url "https://paypal.me/bemyak", "Chat with me" url "https://t.me/@bemyak"]
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

    async fn roll(&self, message: &Message, arg: &str) -> Result<Option<String>, Box<dyn Error>> {
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

    async fn stats(&self, message: &Message) -> Result<Option<String>, Box<dyn Error>> {
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

        info!("{}", msg);

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
}

pub fn get_connector() -> Result<Box<dyn Connector>, Box<dyn Error>> {
    let proxy_url = env::var("roll_bot_http_proxy")
        .or(env::var("ROLL_BOT_HTTP_PROXY"))
        .or(env::var("http_proxy"))
        .or(env::var("HTTP_PROXY"))
        .or(env::var("https_proxy"))
        .or(env::var("HTTPS_PROXY"));

    match proxy_url {
        Ok(proxy_url) => {
            info!("Running with proxy: {}", proxy_url);
            let connector = HttpsConnector::new();
            let proxy = Proxy::new(Intercept::All, proxy_url.parse()?);
            let connector = ProxyConnector::from_proxy(connector, proxy)?;
            Ok(Box::new(HyperConnector::new(
                Client::builder().build(connector),
            )))
        }
        Err(_) => Ok(default_connector()),
    }
}
