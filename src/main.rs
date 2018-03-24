extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

mod telegram;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    telegram::start(&token);
}
