#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod collection;
mod commands;
mod db;
mod fetch;
mod format;
mod metrics;
mod telegram;

use std::error::Error;
use std::{
	env,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde_json::Value as JsonValue;

#[allow(unused_imports)]
use tokio::task;
use tokio::time;

use db::DndDatabase;

pub const PROJECT_URL: &str = "https://gitlab.com/bemyak/roll_bot";
pub const DONATION_URL: &str = "https://ko-fi.com/bemyak";

lazy_static! {
	static ref DB: DndDatabase = {
		if env::var("ROLL_BOT_USE_TEST_DB").is_ok() {
			DndDatabase::new("./test_data/roll_bot.ejdb").unwrap()
		} else {
			DndDatabase::new("./roll_bot.ejdb").unwrap()
		}
	};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	run().await
}

async fn run() -> Result<(), Box<dyn Error>> {
	let log_config = simplelog::ConfigBuilder::new()
		.add_filter_allow_str("roll_bot")
		// .add_filter_allow_str("teloxide")
		.build();
	let log_level = if env::var("ROLL_BOT_USE_TEST_DB").is_ok() {
		simplelog::LevelFilter::Debug
	} else {
		simplelog::LevelFilter::Trace
	};
	simplelog::SimpleLogger::init(log_level, log_config)?;

	task::spawn(async move { metrics::serve_metrics().await });

	task::spawn(async move {
		let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));
		loop {
			interval.tick().await;
			let fetch_result = fetch().await;
			if let Err(err) = fetch_result {
				error!("Error occurred while fetching data: {}", err)
			}
		}
	});

	telegram::start().await;

	Ok(())
}

async fn fetch() -> Result<(), Box<dyn Error + Send + Sync>> {
	let changelog = fetch::fetch_changelog().await?;
	let ver = get_latest_ver(&changelog);
	let cur_ver = DB.get_version().ok().flatten();

	if !should_update(cur_ver.as_ref(), ver) {
		info!(
			"Skipping update, db is running the newest version: {:?}",
			ver
		);
		return Ok(());
	}

	DB.save_collection(changelog, db::VER_COLLECTION_NAME)?;
	let data = fetch::fetch().await?;
	for (collection, items) in data {
		DB.save_collection(items, &collection)?;
	}
	DB.update_cache();
	Ok(())
}

fn should_update(cur_ver: Option<&String>, ver: Option<&str>) -> bool {
	match (cur_ver, ver) {
		(Some(cur_ver), Some(ver)) => cur_ver != ver,
		_ => true,
	}
}

fn get_latest_ver(changelog: &[JsonValue]) -> Option<&str> {
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
