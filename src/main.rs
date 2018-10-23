#[macro_use]
extern crate log;

extern crate ejdb;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate serde_json;
extern crate telegram_bot;
extern crate tokio;
extern crate tokio_core;

use futures::sync::mpsc::{self, Receiver, Sender};
use serde_json::Value;
use std::env;
use tokio::prelude::Future;
use tokio::runtime::Runtime;
use tokio_core::reactor::Core;

mod db;
mod fetcher;
mod telegram;

fn main() {
    env_logger::init();
    let rt = Runtime::new().unwrap();
    let executor = rt.executor();
    let (tx, rx): (Sender<Value>, Receiver<Value>) = mpsc::channel(500000);
    let mut db = db::Storage::init();
    db.start_saver(&executor, rx);
    fetcher::fetch(&executor, tx);
    rt.shutdown_on_idle().wait().unwrap();
    db.stop_saver();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    telegram::start(&token, &mut Core::new().unwrap());
}
