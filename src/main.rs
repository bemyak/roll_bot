#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod collection;
mod db;
mod fetch;
mod format;
mod metrics;
mod telegram;

use std::error::Error;
use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

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

    let use_test_db = env::var("ROLL_BOT_USE_TEST_DB").is_ok();
    let db = if use_test_db {
        Arc::new(DndDatabase::new("./test_data/roll_bot.ejdb")?)
    } else {
        let db = Arc::new(DndDatabase::new("./roll_bot.ejdb")?);
        let fetch_db = db.clone();
        task::spawn(async move {
            fetch_job(fetch_db).await;
        });
        db
    };

    loop {
        let bot = telegram::Bot::new(db.clone()).await?;
        bot.start().await;
        error!("The bot has crashed! Restarting...");
    }
}

async fn fetch_job(db: Arc<DndDatabase>) {
    let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));

    loop {
        interval.tick().await;

        let result = fetch::fetch().await;
        match result {
            Ok(data) => {
                for (collection, items) in data {
                    db.save_collection(items, &collection)
                        .unwrap_or_else(|err| {
                            error!("Error occurred while saving data to DB: {}", err)
                        });
                }
                db.update_cache();
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
