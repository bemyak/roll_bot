use unqlite::UnQLite;
use db;
use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::{Core, Handle};

pub fn start(token: &str, core: &mut Core, db: &UnQLite) {
    let handle = &core.handle();
    let api = Api::configure(token).build(handle).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            dispatcher(&api, &message, &handle, db)
        }
        Ok(())
    });

    println!("Bot is ready");
    core.run(future).unwrap();
}

fn dispatcher(api: &Api, message: &Message, handle: &Handle, db: &UnQLite) {
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            let mut args_iterator = data.as_str().split_whitespace();
            let exec = |args: Vec<&str>, func: fn(&[&str], &Api, &Message, &Handle, db: &UnQLite)| {
                func(&args, &api, &message, handle, db);
            };
            match args_iterator.next() {
                Some("/about") | Some("/help") => exec(vec![], help),
                Some("/item") => exec(vec![], search),
                Some(_) => exec(data.as_str().split_whitespace().collect(), echo),
                None => return,
            };
        }

        _ => return,
    };
}

fn search(_args: &[&str], api: &Api, message: &Message, _handle: &Handle, db: &UnQLite) {
    let result = format!("{:#}", db::search(db, ""));
    api.spawn(message.chat.text(result));
}

fn echo(args: &[&str], api: &Api, message: &Message, _handle: &Handle, _db: &UnQLite) {
    api.spawn(message.chat.text(args.join(" ")));
}

fn help(_args: &[&str], api: &Api, message: &Message, _handle: &Handle, _db: &UnQLite) {
    let help = "I'm the Bot. The Dungeon Bot!
I can help you with your Dungeons & Dragons game.
I can:

/roll - roll a die. By default I have d20, but you can give me any number of dices! ex.: `/roll 2d6 +5`

/mm - search for a monster. I'll look in every book in Candlekeep and find at least one. ex.: `/mm tarasque`

/spell - search for a spell. I'll ask Elminster personally about it. ex.: `/spell fireball`

/item - search for an item. I'll cast Legend Lore spell to know what it is. ex.: `/item bag of holding`

My code is open like your brain for the mind flayer!
You can get it here (code, not brain): https://gitlab.com/bemyak/roll_bot
Suggestions and contributions are welcome.";
    // TODO: ParseMode::Markdown doesn't work for some reason on large text with url
    // api.spawn(message.chat.text(help).parse_mode(ParseMode::Markdown));
    api.spawn(message.chat.text(help));
}
