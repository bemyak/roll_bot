use std::env;
use std::error::Error;

use futures::StreamExt;
use rand::prelude::*;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Message, MessageEntityKind, MessageKind, ParseMode, UpdateKind};

use crate::db::DndDatabase;

const PROJECT_URL: &'static str = "https://gitlab.com/bemyak/roll_bot";

pub struct Bot {
    db: DndDatabase,
    api: Api,
}

impl Bot {
    pub fn new(db: DndDatabase) -> Result<Self, Box<dyn Error>> {
        let token = env::var("ROLL_BOT_TOKEN").unwrap_or_else(|_err| {
            error!("You must provide `ROLL_BOT_TOKEN` environment variable!");
            std::process::exit(1)
        });

        Ok(Self {
            db,
            api: Api::new(token),
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

                            let cmd_result: Result<(), Box<dyn Error>> = match cmd {
                                // WARNING: ParseMode::Markdown doesn't work for some reason on large text with plain-text url
                                "/help" => self.help(message.clone(), arg).await,
                                "/roll" => self.roll(message.clone(), arg).await,
                                _ => self.unknown(message.clone(), cmd).await,
                            };

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
    ) -> Result<(), Box<dyn Error>> {
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
        Ok(())
    }

    async fn unknown(&self, message: Message, cmd: &str) -> Result<(), Box<dyn Error>> {
        self.api
            .send(
                message
                    .chat
                    .text(format!("Errr, I don't know `{}` command yet.", cmd))
                    .parse_mode(ParseMode::Markdown),
            )
            .await?;
        Ok(())
    }

    async fn help(&self, message: Message, _arg: &str) -> Result<(), Box<dyn Error>> {
        let help = format!("Hi! I'm a bot. The Dungeon Bot!
I can help you with your Dungeons&Dragons game (5th edition). I can:

/roll - roll a die. By default I have d20, but you can give me any number of dices! ex.: `/roll 2d6 +5`

/mm - search for a monster. I'll look in every book in Candlekeep and find at least one. ex.: `/mm tarasque`

/spell - search for a spell. I'll ask Elminster personally about it. ex.: `/spell fireball`

/item - search for an item. I'll cast Legend Lore spell to know what it is. ex.: `/item bag of holding`

My code is open like your brain for the mind flayer!
You can get it [here]({}) (code, not brain)
Suggestions and contributions are welcome.", PROJECT_URL);
        self.api
            .send(
                message
                    .chat
                    .text(help)
                    .parse_mode(ParseMode::Markdown)
                    .disable_preview(),
            )
            .await?;
        Ok(())
    }

    async fn roll(&self, message: Message, _arg: &str) -> Result<(), Box<dyn Error>> {
        self.api
            .send(
                message
                    .chat
                    .text(format!("{}", rand::thread_rng().gen_range(0, 20) + 1)),
            )
            .await?;
        Ok(())
    }
}
