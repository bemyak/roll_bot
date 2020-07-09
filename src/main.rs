#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod db;
mod fetch;
mod format;
mod collection;
mod metrics;
mod telegram;

use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[allow(unused_imports)]
use tokio::task;
use tokio::time;

use db::DndDatabase;

pub const PROJECT_URL: &'static str = "https://gitlab.com/bemyak/roll_bot";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_allow_str("roll_bot")
        .build();
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Trace, log_config)?;

    task::spawn(async move { metrics::serve_metrics().await });

    // Use this while testing to avoid unnecessary loading 5e.tools
    // let db = DndDatabase::new("./test_data/roll_bot.ejdb")?;

    // Uncomment this when ready for production use
    let db = DndDatabase::new("./roll_bot.ejdb")?;
    let fetch_db = db.clone();
    task::spawn(async move {
        fetch_job(fetch_db).await;
    });

    let bot = telegram::Bot::new(db.clone()).await?;

    bot.start().await;

    Ok(())
}

async fn fetch_job(mut db: DndDatabase) {
    let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));

    loop {
        interval.tick().await;

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
}

pub fn get_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
