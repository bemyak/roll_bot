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

use serde_json::Value as JsonValue;
#[allow(unused_imports)]
use tokio::task;
use tokio::time;

use db::DndDatabase;

pub const PROJECT_URL: &str = "https://gitlab.com/bemyak/roll_bot";

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
        Arc::new(DndDatabase::new("./roll_bot.ejdb")?)
    };

    let fetch_db = db.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));
        loop {
            interval.tick().await;
            let fetch_result = fetch(fetch_db.clone()).await;
            if let Err(err) = fetch_result {
                error!("Error occurred while fetching data: {}", err)
            }
        }
    });

    loop {
        let bot = telegram::Bot::new(db.clone()).await?;
        bot.start().await;
        error!("The bot has crashed! Restarting...");
    }
}

async fn fetch(db: Arc<DndDatabase>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let changelog = fetch::fetch_changelog().await?;
    let ver = get_latest_ver(&changelog);
    let cur_ver = db.get_version().ok().flatten();

    if !should_update(cur_ver.as_ref(), ver) {
        info!(
            "Skipping update, db is running the newest version: {:?}",
            ver
        );
        return Ok(());
    }

    db.save_collection(changelog, db::VER_COLLECTION_NAME)?;
    let data = fetch::fetch().await?;
    for (collection, items) in data {
        db.save_collection(items, &collection)?;
    }
    db.update_cache();
    Ok(())
}

fn should_update(cur_ver: Option<&String>, ver: Option<&str>) -> bool {
    match (cur_ver, ver) {
        (Some(cur_ver), Some(ver)) => cur_ver != ver,
        _ => true,
    }
}

fn get_latest_ver(changelog: &Vec<JsonValue>) -> Option<&str> {
    let last = changelog.last()?;
    let doc = last.as_object()?;
    let ver = doc.get("ver")?;
    ver.as_str()
}

pub fn get_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
