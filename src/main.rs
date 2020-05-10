#![allow(dead_code)]
extern crate ejdb;
extern crate futures;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde_json;
extern crate simplelog;
extern crate telegram_bot;
extern crate tokio;

mod db;
mod fetch;
mod format;
// mod telegram;

use std::env;
use std::error::Error;

use futures::StreamExt;
use telegram_bot::*;

use db::DndDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default())?;

    let token = env::var("TELEGRAM_BOT_TOKEN")?;

    // let db = DndDatabase::new("./test_data/roll_bot.db")?;

    let db = DndDatabase::new("./roll_bot.db")?;
    fetch::fetch(db.clone()).await?;

    let api = Api::new(token);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                trace!("Message received: {:?}", message);

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }

    Ok(())
}
