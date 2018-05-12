#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
#![allow(unknown_lints)]
#![warn(clippy)]
extern crate futures;
extern crate log;
extern crate serde_json;
extern crate telegram_bot;
extern crate tokio_core;

use db::BotDb;
use fetcher::Fetcher;
use std::env;
use tokio_core::reactor::Core;

mod db;
mod fetcher;
mod telegram;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let db: BotDb = BotDb::init();
    let fetcher = Fetcher::init(Core::new().unwrap(), &db);
    fetcher.fetch();
    telegram::start(&token, &mut Core::new().unwrap());
}
