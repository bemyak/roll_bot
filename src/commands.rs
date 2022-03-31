use std::str::FromStr;
use teloxide::utils::command::{BotCommand, ParseError};

use crate::{
    collection::{Collection, COMMANDS},
    format::roll::roll_dice,
};

#[derive(PartialEq, Debug, Clone)]
pub enum RollBotCommand {
    Help(HelpOptions),
    Roll(String),
    Stats,
    Query((&'static Collection, String)),
}

#[derive(PartialEq, Debug, Clone)]
pub enum HelpOptions {
    None,
    Roll,
}

impl FromStr for HelpOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::None),
            "roll" => Ok(Self::Roll),
            _ => Err(()),
        }
    }
}

impl BotCommand for RollBotCommand {
    fn descriptions() -> String {
        crate::format::telegram::help_message()
    }

    fn parse<N>(s: &str, bot_name: N) -> Result<Self, ParseError>
    where
        N: Into<String>,
    {
        let mut words = s.splitn(2, ' ');
        let mut splited = words.next().expect("First item will always be.").split('@');
        let command_raw = splited.next().expect("First item will always be.");
        let bot = splited.next();
        let bot_name = bot_name.into();
        match bot {
            Some(name) if name == bot_name => {}
            None => {}
            Some(n) => return Err(ParseError::WrongBotName(n.to_string())),
        }
        let args = words.next().unwrap_or("").to_string();

        let cmd = command_raw.strip_prefix('/').unwrap_or(command_raw);
        match cmd {
            "help" | "h" | "about" | "start" => Ok(RollBotCommand::Help(
                HelpOptions::from_str(&args)
                    .map_err(|_| ParseError::UnknownCommand(cmd.to_string()))?,
            )),
            "roll" | "r" => {
                let res = roll_dice(&args)
                    .map_err(Box::new)
                    .map_err(|err| ParseError::IncorrectFormat(err))?;
                Ok(Self::Roll(res))
            }
            "stats" => Ok(Self::Stats),
            _ => {
                if let Some(item) = COMMANDS.get(cmd) {
                    Ok(Self::Query((item, args)))
                } else {
                    Err(ParseError::UnknownCommand(command_raw.to_string()))
                }
            }
        }
    }

    fn bot_commands() -> Vec<teloxide::types::BotCommand> {
        vec![
            teloxide::types::BotCommand::new("roll", "Roll a dice (d20 by default)"),
            teloxide::types::BotCommand::new("spell", "Search for a spell"),
            teloxide::types::BotCommand::new("item", "Search for an item"),
            teloxide::types::BotCommand::new("monster", "Search for a monster"),
            teloxide::types::BotCommand::new("help", "Show help"),
        ]
    }
}
