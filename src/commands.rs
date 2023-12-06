use std::str::FromStr;
use teloxide::utils::command::{BotCommands, CommandDescription, CommandDescriptions, ParseError};

use crate::{
	collection::{Collection, COMMANDS},
	format::{roll::roll_dice, utils::HtmlEscapable},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RollBotCommands {
	Help(HelpOptions),
	Roll(String),
	// Stats,
	Query((&'static Collection, String)),
	Echo(String),
	Error(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

impl BotCommands for RollBotCommands {
	fn descriptions() -> CommandDescriptions<'static> {
		CommandDescriptions::new(&[
			CommandDescription {
				prefix: "/",
				command: "roll",
				description: "Roll a dice (d20 by default)",
			},
			CommandDescription {
				prefix: "/",
				command: "spell",
				description: "Search for a spell",
			},
			CommandDescription {
				prefix: "/",
				command: "item",
				description: "Search for an item",
			},
			CommandDescription {
				prefix: "/",
				command: "monster",
				description: "Search for a monster",
			},
			CommandDescription {
				prefix: "/",
				command: "help",
				description: "Show help",
			},
		])
	}

	fn parse(s: &str, bot_name: &str) -> Result<Self, ParseError> {
		let mut words = s.splitn(2, |c| c == ' ' || c == '\n');
		let mut splited = words
			.next()
			.expect("Command always starts with a slash (/)")
			.split('@');
		let command_raw = splited.next().expect("First item will always be.");
		let bot = splited.next();
		match bot {
			Some(name) if name == bot_name => {}
			None => {}
			Some(n) => return Err(ParseError::WrongBotName(n.to_string())),
		}
		let args = words.next().unwrap_or("").to_string();

		let cmd = command_raw
			.strip_prefix('/')
			.unwrap_or(command_raw)
			.to_lowercase();
		match cmd.as_str() {
			"help" | "h" | "about" | "start" => Ok(RollBotCommands::Help(
				HelpOptions::from_str(&args).map_err(|_| ParseError::UnknownCommand(cmd))?,
			)),
			"roll" | "r" => roll_dice(&args)
				.map(Self::Roll)
				.or_else(|err| Ok(Self::Error(err.to_string()))),
			// "stats" => Ok(Self::Stats),
			"echo" => Ok(Self::Echo(args.escape_html())),
			_ => {
				if let Some(item) = COMMANDS.get(cmd.as_str()) {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
	cmd: &'a str,
	arg: Option<&'a str>,
}

peg::parser! {
	grammar command_parser() for str {
		use peg::ParseLiteral;

		rule _()
		= [' ' | '\t']*

		rule __()
		= [' ' | '\t']+ / ['\n']*<1,>

		rule text() -> &'input str
		= _ text:$(!"/" !"\n" [_]+) _ {
			text
		}

		pub rule command(bot_name: &str) -> Command<'input>
		= _ "/" cmd:$([ 'a'..='z' | 'A'..='Z']+) ("@" ##parse_string_literal(bot_name))? arg:$(__ text())? {
			Command {
				cmd,
				arg: arg.map(|arg| arg.trim()),
			}
		}

		// rule not_command(bot_name: &str)
		// = (!command(bot_name) [_])

		// pub rule commands(bot_name: &str) -> Vec<Command<'input>>
		// = not_command(bot_name)* c:(command(bot_name) ** (not_command(bot_name)*)) not_command(bot_name)* {
		// 	c
		// }

		rule commands_skip(bot_name: &str) -> Command<'input>
		= &(command(bot_name)) c:command(bot_name) {
			c
		}

		pub rule commands(bot_name: &str) -> Vec<Command<'input>>
		= commands_skip(bot_name)+
	}
}

#[test]
fn test_command_parser_rule() {
	let bot_name = "roll_bot";
	assert_eq!(
		command_parser::command("/r", bot_name),
		Ok(Command {
			cmd: "r",
			arg: None,
		})
	);

	assert_eq!(
		command_parser::command("/roll", bot_name),
		Ok(Command {
			cmd: "roll",
			arg: None,
		})
	);

	assert_eq!(
		command_parser::command("/r test test test", bot_name),
		Ok(Command {
			cmd: "r",
			arg: Some("test test test"),
		})
	);

	assert_eq!(
		command_parser::command("/r    test test test     ", bot_name),
		Ok(Command {
			cmd: "r",
			arg: Some("test test test"),
		})
	);

	assert_eq!(
		command_parser::command("/r\ntest", bot_name),
		Ok(Command {
			cmd: "r",
			arg: Some("test"),
		})
	);

	// assert!(command_parser::command("/r\n\ntest", bot_name).is_err());

	#[allow(clippy::invisible_characters)]
	{
		assert_eq!(
			command_parser::command("/roll 1d20 5d30 ­ ⛤", bot_name),
			Ok(Command {
				cmd: "roll",
				arg: Some("1d20 5d30 ­ ⛤"),
			})
		);
	}
}

#[test]
fn test_command_parser_rules() {
	let bot_name = "roll_bot";
	assert_eq!(
		command_parser::commands("/r", bot_name),
		Ok(vec![Command {
			cmd: "r",
			arg: None,
		}])
	);

	// assert_eq!(
	// 	command_parser::commands("/r /r", bot_name),
	// 	Ok(vec![
	// 		Command {
	// 			cmd: "r",
	// 			arg: None,
	// 		},
	// 		Command {
	// 			cmd: "r",
	// 			arg: None,
	// 		}
	// 	])
	// );

	// assert_eq!(
	// 	command_parser::commands("garbage text /r /r", bot_name),
	// 	Ok(vec![
	// 		Command {
	// 			cmd: "r",
	// 			arg: None,
	// 		},
	// 		Command {
	// 			cmd: "r",
	// 			arg: None,
	// 		}
	// 	])
	// );
}
