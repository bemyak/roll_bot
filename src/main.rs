#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
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
    let db: BotDb = BotDb::init();
    let core: Core = Core::new().unwrap();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let handle = core.handle();
    let fetcher = Fetcher::init(core, &handle, db);
    fetcher.fetch();
    // telegram::start(&token, &mut core, &handle);
}
