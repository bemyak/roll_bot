use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::Instant;

use futures::StreamExt;
use hyper::Client;
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_rustls::HttpsConnector;
use rand::prelude::*;
use regex::Regex;
use telegram_bot::{
    connector::{default_connector, hyper::HyperConnector, Connector},
    prelude::*,
    Api, Message, MessageEntityKind, MessageKind, ParseMode, UpdateKind,
};

use crate::db::DndDatabase;
use crate::format::*;
use crate::PROJECT_URL;

pub struct Bot {
    db: DndDatabase,
    api: Api,
    dice_regex: Regex,
    cache: HashMap<String, Vec<String>>,
    cache_timestamp: Instant,
}

impl Bot {
    pub fn new(db: DndDatabase) -> Result<Self, Box<dyn Error>> {
        let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
            error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
            std::process::exit(1)
        });

        let cache = db.get_cache();

        let connector = get_connector()?;

        Ok(Self {
            db,
            api: Api::with_connector(token, connector),
            dice_regex: Regex::new(r"(?P<num>\d+)?(d|к|д)(?P<face>\d+)").unwrap(),
            cache,
            cache_timestamp: Instant::now(),
        })
    }

    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        info!("Starting roll_bot...");
        let mut stream = self.api.stream();

        while let Some(update) = stream.next().await {
            let update = update?;
            trace!("Received message: {:#?}", update);
            // TODO: Make it work with UpdateKind::EditedMessage as well
            if let UpdateKind::Message(message) = update.kind {
                self.process_message(message).await?;
            }
        }

        Ok(())
    }

    async fn process_message(&self, message: Message) -> Result<(), Box<dyn Error>> {
        match message.clone().kind {
            MessageKind::Text { data, entities } => {
                let mut entities_iter = entities.into_iter().peekable();

                while let Some(entity) = entities_iter.next() {
                    match entity.kind {
                        MessageEntityKind::BotCommand => {
                            let cmd_start = entity.offset as usize;
                            let cmd_end = (entity.offset + entity.length) as usize;
                            let cmd = &data[cmd_start..cmd_end];

                            let arg_start = cmd_end;
                            let arg_end = entities_iter
                                .peek()
                                .map_or(data.len(), |next_entity| next_entity.offset as usize);
                            let arg = &data[arg_start..arg_end].trim();

                            let cmd_result = match cmd {
                                // WARNING: ParseMode::Markdown doesn't work for some reason on large text with plain-text url
                                // The returned string value is used to log request-response pair into the database
                                "/help" => self.help(message.clone(), arg).await,
                                "/roll" => self.roll(message.clone(), arg).await,
                                "/stats" => self.stats(message.clone()).await,
                                _ => self.unknown(message.clone(), cmd).await,
                            };

                            self.db.log_message(&message, &cmd_result);

                            if let Err(err) = cmd_result {
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
            "Ooops! An error occurred :(\nPlease, [submit a bug report]({})",
            url
        );
        // TODO: Send email automatically to Gitlab Service Desk
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

    async fn unknown(&self, message: Message, cmd: &str) -> Result<Option<String>, Box<dyn Error>> {
        self.api
            .send(
                message
                    .chat
                    .text(format!("Errr, I don't know `{}` command yet.", cmd))
                    .parse_mode(ParseMode::Markdown),
            )
            .await?;
        Ok(None)
    }

    async fn help(&self, message: Message, _arg: &str) -> Result<Option<String>, Box<dyn Error>> {
        let help = help_message();

        self.api
            .send(
                message
                    .chat
                    .text(help)
                    .parse_mode(ParseMode::Markdown)
                    .disable_preview(),
            )
            .await?;
        Ok(None)
    }

    async fn roll(&self, message: Message, _arg: &str) -> Result<Option<String>, Box<dyn Error>> {
        let msg = format!("{}", rand::thread_rng().gen_range(0, 20) + 1);

        self.api.send(message.chat.text(msg.clone())).await?;
        Ok(Some(msg))
    }

    async fn stats(&self, message: Message) -> Result<Option<String>, Box<dyn Error>> {
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
            "*Table stats*\n{}\n\n*Usage stats*\n{}\n\nLast database update `{}` ago",
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
        .or(env::var("HTTPS_PROXY"))
        .or(env::var("ftp_proxy"))
        .or(env::var("FTP_PROXY"));

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
