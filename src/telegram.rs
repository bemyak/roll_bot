use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::{Core, Handle};

pub fn start(token: &str) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            dispatcher(api.clone(), message, &handle)
        }
        Ok(())
    });

    core.run(future).unwrap();
}

fn dispatcher(api: Api, message: Message, handle: &Handle) {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let mut args_iterator = data.as_str().split_whitespace();
            let exec = |args: Vec<&str>, func: fn(Vec<&str>, Api, &Message, &Handle)| {
                func(args, api, &message, handle);
            };
            match args_iterator.next() {
                Some("/about") | Some("/help") => exec(vec![], help),
                Some(_) => exec(data.as_str().split_whitespace().collect(), echo),
                None => return,
            };
        }

        _ => return,
    };
}

fn echo(args: Vec<&str>, api: Api, message: &Message, _handle: &Handle) {
    api.spawn(message.chat.text(args.join(" ")));
}

fn help(_args: Vec<&str>, api: Api, message: &Message, _handle: &Handle) {
    api.spawn(message.chat.text("I'm the Bot. The Dungeon Bot!
I can help you with your Dungeons & Dragons game.
I can:

/roll - roll a die. By default I have d20, but you can give me any number of dices! ex.: `/roll 2d6 +5`

/mm - search for a monster. I'll look in every book in Candlekeep and find at least one. ex.: `/mm tarasque`

/spell - search for a spell. I'll ask Elminster personally about it. ex.: `/spell fireball`

/item - search for an item. I'll cast Legend Lore spell to know what it is. ex.: `/item bag of holding`

My code is open like your brain for the mind slayer!
You can get it here (code, not brain): https://gitlab.com/bemyak/roll-bot
Suggestions and contributions are welcome.  
"));
}
