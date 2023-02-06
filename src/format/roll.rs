#![allow(clippy::redundant_closure_call)]

use std::fmt::Display;
use std::{num::ParseIntError, str::FromStr};

use peg::error::ParseError;
use peg::str::LineCol;
use rand::prelude::*;
use thiserror::Error;

use crate::format::utils::zalgofy;

const PENTAGRAM: &str = "⛧";

#[derive(Error, Debug)]
pub enum DieFormatError {
	#[error("Wow, that was a lot of text! Too bad I'm too lazy to read it :)")]
	TooLongText,

	#[error("I don't have that many dices!")]
	TooManyRolls,
	#[error("{0}")]
	ParseError(&'static str),
}

pub fn roll_dice(msg: &str) -> Result<String, DieFormatError> {
	let response = roll_results(msg)?
		.iter()
		.map(|roll| match &roll.comment {
			Some(comment) => format!(
				"{}: {} = {}",
				comment,
				roll.expression,
				roll.expression.calc()
			),
			None => format!("{} = {}", roll.expression, roll.expression.calc()),
		})
		.collect::<Vec<_>>()
		.join("\n");

	if response.is_empty() {
		warn!("Cannot parse: {}", msg);
		Ok("Err, sorry, I can't roll that. Maybe you need some /help ?".to_owned())
	} else {
		Ok(response)
	}
}

pub fn roll_results(msg: &str) -> Result<Vec<RollLine>, DieFormatError> {
	if msg.len() > u16::MAX as usize {
		return Err(DieFormatError::TooLongText);
	}
	let rolls = roll_parser::expressions(msg).map_err(DieFormatError::from)?;
	if rolls.len() > 20 {
		return Err(DieFormatError::TooManyRolls);
	}
	Ok(rolls
		.into_iter()
		.map(|rolls| {
			if let RollLine {
				expression: Expression::Value(Operand::Num(num)),
				comment,
			} = rolls
			{
				let substitution = Expression::Value(Operand::Dice(Dice::new_num(num, 20)));
				RollLine {
					expression: substitution,
					comment,
				}
			} else {
				rolls
			}
		})
		.collect())
}

impl From<ParseError<LineCol>> for DieFormatError {
	fn from(err: ParseError<LineCol>) -> Self {
		let e = err
			.expected
			.tokens()
			.find(|s| s.starts_with("Nope") || s.starts_with("Wow"))
			.unwrap_or("Can't parse your message, sorry");
		Self::ParseError(e)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
	Dice(Dice),
	Num(u16),
}

impl Display for Operand {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operand::Dice(d) => {
				write!(f, "{d}")
			}
			Operand::Num(n) => {
				write!(f, "{n}")
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiceNum {
	Advantage,
	Disadvantage,
	Num(u16),
}

impl FromStr for DiceNum {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"+" => DiceNum::Advantage,
			"-" => DiceNum::Disadvantage,
			_ => DiceNum::Num(s.parse()?),
		})
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiceFace {
	Num(u16),
	Percentile,
	Fudge,
	Zalgo,
}

impl Display for DiceFace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DiceFace::Num(num) => write!(f, "{num}"),
			DiceFace::Percentile => write!(f, "%"),
			DiceFace::Fudge => write!(f, "F"),
			DiceFace::Zalgo => write!(f, "{PENTAGRAM}"),
		}
	}
}

impl DiceFace {
	fn get_min_value(&self) -> i32 {
		match self {
			DiceFace::Fudge => -1,
			_ => 1,
		}
	}
	fn get_max_value(&self) -> i32 {
		match self {
			DiceFace::Num(num) => *num as i32,
			DiceFace::Fudge => 1,
			DiceFace::Percentile => 100,
			DiceFace::Zalgo => 20,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiceSelector {
	KeepHigh(u16),
	KeepLow(u16),
	DropHigh(u16),
	DropLow(u16),
}

impl Display for DiceSelector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DiceSelector::KeepHigh(n) => write!(f, "kh{n}"),
			DiceSelector::KeepLow(n) => write!(f, "kl{n}"),
			DiceSelector::DropHigh(n) => write!(f, "dh{n}"),
			DiceSelector::DropLow(n) => write!(f, "dl{n}"),
		}
	}
}

#[derive(Debug)]
pub struct Dice {
	pub num: u16,
	pub face: DiceFace,
	pub selectors: Vec<DiceSelector>,
	results: Vec<i32>,
	total: i64,
}

impl Dice {
	pub fn new(num: u16, face: DiceFace, selectors: Vec<DiceSelector>) -> Self {
		let mut results: Vec<i32> = (0..num)
			.map(|_| rand::thread_rng().gen_range(face.get_min_value()..face.get_max_value() + 1))
			.collect();
		results.sort();
		let results_full = results.clone();
		for selector in &selectors {
			results = match selector {
				DiceSelector::KeepHigh(n) => {
					results.into_iter().rev().take(*n as usize).rev().collect()
				}
				DiceSelector::KeepLow(n) => results.into_iter().take(*n as usize).collect(),
				DiceSelector::DropHigh(n) => {
					results.into_iter().rev().skip(*n as usize).rev().collect()
				}
				DiceSelector::DropLow(n) => results.into_iter().skip(*n as usize).collect(),
			};
		}
		let mut total = 0;
		for r in &results {
			total += *r as i64;
		}
		Self {
			num,
			face,
			selectors,
			results: results_full,
			total,
		}
	}

	pub fn new_num(num: u16, face: u16) -> Self {
		Self::new(num, DiceFace::Num(face), vec![])
	}

	pub fn new_adv() -> Self {
		Self::new(2, DiceFace::Num(20), vec![DiceSelector::KeepHigh(1)])
	}

	pub fn new_disadv() -> Self {
		Self::new(2, DiceFace::Num(20), vec![DiceSelector::KeepLow(1)])
	}
}

impl Default for Dice {
	fn default() -> Self {
		Self::new(1, DiceFace::Num(20), vec![])
	}
}

impl Display for Dice {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let face_str = self.face.to_string();
		let face_int = self.face.get_min_value();
		match (self.num, self.selectors.as_slice()) {
			(num, []) => write!(f, "<code>{num}d{face_str}</code>")?,
			(2, [DiceSelector::KeepHigh(1)]) => {
				write!(f, "<code>d{face_str} with advantage</code>")?
			}
			(2, [DiceSelector::KeepLow(1)]) => {
				write!(f, "<code>d{face_str} with disadvantage</code>")?
			}

			(num, selectors) => {
				write!(f, "<code>{num}d{face_str}")?;
				for selector in selectors {
					write!(f, "{selector}")?;
				}
				write!(f, "</code>")?;
			}
		};

		let roll_results = if matches!(self.face, DiceFace::Zalgo) {
			let mut r = self
				.results
				.iter()
				.map(|roll| roll.to_string())
				.collect::<Vec<_>>()
				.join(",");
			zalgofy(&mut r);
			r
		} else {
			self.results
				.iter()
				.map(|roll| {
					if *roll == 1 || *roll == face_int {
						format!("<b>{roll}</b>")
					} else {
						format!("{roll}")
					}
				})
				.collect::<Vec<_>>()
				.join(",")
		};
		write!(f, " [{roll_results}]")
	}
}

impl PartialEq for Dice {
	fn eq(&self, other: &Self) -> bool {
		self.num == other.num && self.face == other.face
	}
}

impl Eq for Dice {}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
	Value(Operand),
	Plus(Box<Expression>, Box<Expression>),
	Minus(Box<Expression>, Box<Expression>),
	Multiply(Box<Expression>, Box<Expression>),
	Divide(Box<Expression>, Box<Expression>),
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Value(v) => {
				write!(f, "{v}")
			}
			Expression::Plus(a, b) => {
				write!(f, "{a} + {b}")
			}
			Expression::Minus(a, b) => {
				write!(f, "{a} - {b}")
			}
			Expression::Multiply(a, b) => {
				match **a {
					Expression::Plus(_, _) | Expression::Minus(_, _) => {
						write!(f, "({a})")?;
					}
					_ => {
						write!(f, "{a}")?;
					}
				}
				write!(f, " × ")?;
				match **b {
					Expression::Plus(_, _) | Expression::Minus(_, _) => {
						write!(f, "({b})")?;
					}
					_ => {
						write!(f, "{b}")?;
					}
				}
				Ok(())
			}
			Expression::Divide(a, b) => {
				match **a {
					Expression::Plus(_, _) | Expression::Minus(_, _) => {
						write!(f, "({a})")?;
					}
					_ => {
						write!(f, "{a}")?;
					}
				}
				write!(f, " ÷ ")?;
				match **b {
					Expression::Plus(_, _) | Expression::Minus(_, _) => {
						write!(f, "({b})")?;
					}
					_ => {
						write!(f, "{b}")?;
					}
				}
				Ok(())
			}
		}
	}
}

impl Default for Expression {
	fn default() -> Self {
		Self::Value(Operand::Dice(Dice::default()))
	}
}

impl Expression {
	pub fn calc(&self) -> i64 {
		match self {
			Expression::Value(operand) => match operand {
				Operand::Dice(d) => d.total,
				Operand::Num(n) => *n as i64,
			},
			Expression::Plus(a, b) => a.calc() + b.calc(),
			// TODO: Fix subtraction
			Expression::Minus(a, b) => a.calc().checked_sub(b.calc()).unwrap_or_default(),
			Expression::Multiply(a, b) => a.calc() * b.calc(),
			Expression::Divide(a, b) => (a.calc() as f32 / b.calc() as f32).round() as i64,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct RollLine {
	pub expression: Expression,
	pub comment: Option<String>,
}

impl RollLine {
	fn new(expression: Expression, comment: Option<String>) -> Self {
		match comment {
			None => Self {
				expression,
				comment,
			},
			Some(comment) => Self {
				expression,
				comment: Some(comment),
			},
		}
	}
}

peg::parser! {
	grammar roll_parser() for str {

		rule _()
		= [' ' | '\n' | '\t']*

		rule __()
		= [' ' | '\n' | '\t']+

		rule num() -> u16
		= num:$(['0'..='9']+)
			{? num.parse().or(Err("Wow, that's a big number!")) }

		rule dice_num() -> DiceNum
		= num:$(num() / "+" / "-")
			{? num.parse().or(Err("Wow, an error occurred, which shouldn't happen 🤔. Are you happy?")) }

		rule dice_face() -> DiceFace
		= num:$(num() / "%" / "⛧" / "F")
			{?
				Ok(match num {
					"%" => DiceFace::Percentile,
					"F" => DiceFace::Fudge,
					PENTAGRAM | "0" => DiceFace::Zalgo,
					_ => DiceFace::Num(
						num.parse().or(Err("Wow, an error occurred, which shouldn't happen 🤔. Are you happy?"))?
					),
				})
			}

		rule dice_selector() -> DiceSelector
		= op:$("kh" / "kl" / "dh" / "dl") num:num()?
			{
				let num = num.unwrap_or(1);
				match op {
					"kh" => DiceSelector::KeepHigh(num),
					"kl" => DiceSelector::KeepLow(num),
					"dh" => DiceSelector::DropHigh(num),
					"dl" => DiceSelector::DropLow(num),
					_ => unreachable!()
				}
			}

		rule dice_selector_short() -> DiceSelector
		= op:$("H"/ "L")
			{
				match op {
					"H" => DiceSelector::KeepHigh(1),
					"L" => DiceSelector::KeepLow(1),
					_ => unreachable!(),
				}
			}

		pub rule dice() -> Dice
		= num:dice_num()? ['d' | 'D' | 'к' | 'д'] face:dice_face() selectors:(dice_selector() / dice_selector_short())*
			{?
				let dice_num = num.unwrap_or(DiceNum::Num(1));
				let (dice_num, selectors) = match (dice_num, selectors.as_slice()) {
					(DiceNum::Advantage, []) => (2, vec![DiceSelector::KeepHigh(1)]),
					(DiceNum::Disadvantage, []) => (2, vec![DiceSelector::KeepLow(1)]),
					(DiceNum::Num(200..), _) => return Err("Nope, I don't have that many dices!"),
					(DiceNum::Num(num), _) => (num, selectors),
					_ => return Err("Nope, that doesn't make any sense"),
				};
				if face.get_min_value() > 1000 {
					return Err("Nope, I don't have that kind of dice!")
				}
				Ok(Dice::new(dice_num, face, selectors))
			}

		rule dice_operand() -> Operand
		= dice:dice()
			{ Operand::Dice(dice) }

		rule num_operand() -> Operand
		= num:num()
			{ Operand::Num(num) }

		pub rule operand() -> Operand
		= dice_operand() / num_operand()

		rule full_expression() -> Expression
		= precedence! {
			x:(@) _ "+" _ y:@ { Expression::Plus(Box::new(x), Box::new(y)) }
			x:(@) _ "-" _ y:@ { Expression::Minus(Box::new(x), Box::new(y)) }
			--
			x:(@) _ ("*" / "×") _ y:@ { Expression::Multiply(Box::new(x), Box::new(y)) }
			x:(@) _ "÷" _ y:@ { Expression::Divide(Box::new(x), Box::new(y)) }
			--
			n:operand() { Expression::Value(n) }
			"(" _ e:full_expression() _ ")" { e }
		}

		rule short_adv() -> Expression
		= sign:$['+' | '-' ] {
			match sign {
				"+" => Expression::Value(Operand::Dice(Dice::new_adv())),
				"-" => Expression::Value(Operand::Dice(Dice::new_disadv())),
				_ => unreachable!()
			}
		}

		rule short_bonus() -> Expression
		= sign:$['+' | '-' | '*' | '×' | '÷' ] _ num:num() {
			let v1 = Box::new(Expression::Value(Operand::Dice(Dice::default())));
			let v2 = Box::new(Expression::Value(Operand::Num(num)));
			match sign {
				"+" => Expression::Plus(v1,v2),
				"-" => Expression::Minus(v1,v2),
				"*" | "×" => Expression::Multiply(v1,v2),
				"÷" => Expression::Divide(v1,v2),
				_ =>
					unreachable!("Unknown sign {}", sign)
			}
		}

		pub rule expression() -> Expression
		= full_expression() / short_bonus() / short_adv()

		rule comment() -> Option<String>
		= !expression() c:$(([_] !expression())+) {
			const TRIM: [char; 5] = ['\\', ',', ';', '.', ':'];
			let c = c.trim_matches(|c: char| c.is_whitespace() || TRIM.contains(&c) );
			if c.is_empty() {
				None
			} else {
				Some(c.to_owned())
			}
		}

		rule expression_with_comment() -> RollLine
		= e:expression() __ c:comment() {
			RollLine::new(e, c)
		}

		rule only_comment() -> Vec<RollLine>
		= c:comment() {
			let expression = Expression::default();
			vec![RollLine::new(expression, c)]
		}

		rule expression_without_comment() -> RollLine
		= e:expression() {
			RollLine::new(e, None)
		}

		rule roll_line() -> RollLine
		= expression_with_comment() / expression_without_comment()

		rule nothing() -> Vec<RollLine>
		= ![_] {
			vec![RollLine::default()]
		}

		rule traced<T>(e: rule<T>) -> T
		= &(input:$([_]*) {
				#[cfg(feature = "trace")]
				println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
			})
			e:e()? {?
				#[cfg(feature = "trace")]
				println!("[PEG_TRACE_STOP]");
				e.ok_or("")
	}

		pub rule expressions() -> Vec<RollLine>
		= traced(<roll_line() ++ __ / only_comment() / nothing()>)

	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_parse_operand() {
		assert_eq!(
			roll_parser::operand("1d20"),
			Ok(Operand::Dice(Dice::default()))
		);
		assert_eq!(roll_parser::operand("5"), Ok(Operand::Num(5)));
		assert_eq!(
			roll_parser::operand("+d20"),
			Ok(Operand::Dice(Dice::new_adv()))
		);
	}

	#[test]
	fn test_full_notation() {
		assert_eq!(
			roll_parser::dice("10d20kh4kl3dh2dl1"),
			Ok(Dice {
				num: 10,
				face: DiceFace::Num(20),
				selectors: vec![
					DiceSelector::KeepHigh(4),
					DiceSelector::KeepLow(3),
					DiceSelector::DropHigh(2),
					DiceSelector::DropLow(1),
				],
				results: vec![],
				total: 0,
			})
		);
	}

	#[test]
	fn test_percentile() {
		assert_eq!(
			roll_parser::dice("10d%"),
			Ok(Dice {
				num: 10,
				face: DiceFace::Percentile,
				selectors: vec![],
				results: vec![],
				total: 0,
			})
		);
	}

	#[test]
	fn test_fudge() {
		assert_eq!(
			roll_parser::dice("4dF"),
			Ok(Dice {
				num: 4,
				face: DiceFace::Fudge,
				selectors: vec![],
				results: vec![],
				total: 0,
			})
		);
	}

	#[test]
	fn test_parse_expression() {
		assert_eq!(
			roll_parser::expression("1d20 + 5"),
			Ok(Expression::Plus(
				Box::new(Expression::Value(Operand::Dice(Dice::default()))),
				Box::new(Expression::Value(Operand::Num(5)))
			))
		);
		assert_eq!(
			roll_parser::expression("+d20"),
			Ok(Expression::Value(Operand::Dice(Dice::new_adv())))
		);
		assert_eq!(
			roll_parser::expression("d20"),
			Ok(Expression::Value(Operand::Dice(Dice::default())))
		);
		assert_eq!(
			roll_parser::expression("d4"),
			Ok(Expression::Value(Operand::Dice(Dice::new_num(1, 4))))
		);
		assert_eq!(
			roll_parser::expression("d6+d4+3"),
			Ok(Expression::Plus(
				Box::new(Expression::Plus(
					Box::new(Expression::Value(Operand::Dice(Dice::new_num(1, 6)))),
					Box::new(Expression::Value(Operand::Dice(Dice::new_num(1, 4))))
				)),
				Box::new(Expression::Value(Operand::Num(3)))
			))
		);
		assert_eq!(
			roll_parser::expression("+d20+5"),
			Ok(Expression::Plus(
				Box::new(Expression::Value(Operand::Dice(Dice::new_adv()))),
				Box::new(Expression::Value(Operand::Num(5)))
			))
		);
		assert_eq!(
			roll_parser::expression("+"),
			Ok(Expression::Value(Operand::Dice(Dice::new_adv())))
		);
		assert_eq!(
			roll_parser::expression("+5"),
			Ok(Expression::Plus(
				Box::new(Expression::Value(Operand::Dice(Dice::default()))),
				Box::new(Expression::Value(Operand::Num(5)))
			))
		);
	}

	#[test]
	fn test_parse_expressions() {
		assert!(roll_parser::expressions("10000000d5").is_err());

		assert_eq!(
			roll_parser::expressions("1d20 + 5"),
			Ok(vec![RollLine {
				expression: Expression::Plus(
					Box::new(Expression::Value(Operand::Dice(Dice::default()))),
					Box::new(Expression::Value(Operand::Num(5)))
				),
				comment: None,
			}])
		);

		assert_eq!(
			roll_parser::expressions("1d20 1d6"),
			Ok(vec![
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::default())),
					comment: None,
				},
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::new_num(1, 6))),
					comment: None,
				}
			])
		);

		assert_eq!(
			roll_parser::expressions("+ + +"),
			Ok(vec![
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::new_adv())),
					comment: None,
				},
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::new_adv())),
					comment: None,
				},
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::new_adv())),
					comment: None,
				},
			])
		);
	}

	#[test]
	fn test_display_expression() {
		let expr = roll_parser::expression("1d10").unwrap();
		print!("{expr}");
		println!(" = {}", expr.calc());
	}

	#[test]
	fn test_errors() {
		let expr = roll_parser::expression("9999999d200");
		assert!(expr.is_err());
	}

	#[test]
	fn test_comment() {
		let expr = roll_parser::expressions("d20 + 5 to sneak the target");
		assert_eq!(
			expr,
			Ok(vec![RollLine {
				expression: Expression::Plus(
					Box::new(Expression::Value(Operand::Dice(Dice::default()))),
					Box::new(Expression::Value(Operand::Num(5)))
				),
				comment: Some("to sneak the target".to_owned())
			}])
		);
		// assert!(expr.is_ok());
		// let expr = expr.unwrap();
		// let expr = expr.get(0).unwrap();
		// assert_eq!(expr.comment, Some("to sneak the target".to_owned()));
	}

	#[test]
	fn test_mixed_comment() {
		let expr = roll_parser::expressions("d20 to sneak the target 2d6 damage");
		println!("{expr:?}");
		assert!(expr.is_ok());
		let expr = expr.unwrap();
		assert_eq!(
			expr,
			vec![
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::default())),
					comment: Some("to sneak the target".to_owned()),
				},
				RollLine {
					expression: Expression::Value(Operand::Dice(Dice::new_num(2, 6))),
					comment: Some("damage".to_owned()),
				}
			]
		);
	}

	#[test]
	fn test_empty_comment() {
		let expr = roll_parser::expressions("to sneak the target");
		assert!(expr.is_ok());
		let expr = expr.unwrap();
		assert_eq!(
			expr,
			vec![RollLine {
				expression: Expression::Value(Operand::Dice(Dice::default())),
				comment: Some("to sneak the target".to_owned()),
			}]
		);
	}
}
