extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    telegram::start(&token);
}

mod telegram {
    use futures::Stream;
    use telegram_bot::*;
    use tokio_core::reactor::Core;
    pub fn start(token: &str) {
        let mut core = Core::new().unwrap();

        let api = Api::configure(token).build(core.handle()).unwrap();

        let future = api.stream().for_each(|update| {
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    println!("<{}>: {}", &message.from.first_name, data);
                    api.spawn(message.text_reply(data.clone()));
                }
            }

            Ok(())
        });

        core.run(future).unwrap();
    }
}
