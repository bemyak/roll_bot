#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
extern crate futures;
extern crate log;
extern crate serde_json;
extern crate telegram_bot;
extern crate tokio_core;

use db::BotDb;
use std::env;
use tokio_core::reactor::Core;

mod db;
mod fetcher;
mod telegram;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let db = BotDb::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    fetcher::fetch(&mut core, &handle, &db);
    telegram::start(&token, &mut core, &handle);
}
