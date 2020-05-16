#![allow(dead_code)]
extern crate ejdb;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate simplelog;
extern crate telegram_bot;
extern crate tokio;

mod db;
mod fetch;
mod format;
mod telegram;

use std::error::Error;

use db::DndDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_allow_str("roll_bot")
        .build();
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Trace, log_config)?;

    // Use this while testing to avoid unnecessary loading 5e.tools
    let db = DndDatabase::new("/tmp/roll_bot.db")?;

    // Uncomment this when ready for production use
    // let db = DndDatabase::new("./roll_bot.db")?;
    fetch(db.clone()).await;
    info!("{}", db.get_stats());
    fetch(db.clone()).await;
    info!("{}", db.get_stats());

    // let bot = telegram::Bot::new(db)?;
    // bot.start().await?;

    Ok(())
}

async fn fetch(db: DndDatabase) {
    let result = fetch::fetch().await;
    match result {
        Ok(data) => {
            for (collection, items) in data {
                db.save_collection(items, collection).unwrap_or_else(|err| {
                    error!("Error occurred while saving data to DB: {}", err)
                });
            }
        }
        Err(err) => error!("Error occurred while fetching data: {}", err),
    }
}
