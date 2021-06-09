use std::fmt::Display;
use std::{num::ParseIntError, str::FromStr};

use rand::prelude::*;
use thiserror::Error;

use crate::format::utils::zalgofy;

#[derive(Error, Debug)]
pub enum DieFormatError {
    #[error("Wow, that was a lot of text! Too bad I'm too lazy to read it :)")]
    TooLongText,
    #[error("Sorry, can't parse this")]
    ParseError,
}

pub fn roll_dice(msg: &str) -> Result<String, DieFormatError> {
    let response = roll_results(msg)?
        .iter()
        .map(|roll| match &roll.comment {
            Some(comment) => format!(
                "{}: {} = {}",
                comment,
                roll.expression.to_string(),
                roll.expression.calc()
            ),
            None => format!(
                "{} = {}",
                roll.expression.to_string(),
                roll.expression.calc()
            ),
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
    roll_parser::expressions(msg)
        .map_err(|_err| DieFormatError::ParseError)
        .map(|rolls| {
            if let [RollLine {
                expression: Expression::Value(Operand::Num(num)),
                comment,
            }] = &*rolls
            {
                let substitution =
                    Expression::Value(Operand::Dice(Dice::new(DiceNum::Num(*num), 20)));
                vec![RollLine {
                    expression: substitution,
                    comment: comment.clone(),
                }]
            } else {
                rolls
            }
        })
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Dice(Dice),
    Num(u16),
}

impl Operand {
    pub fn dice(num: DiceNum, face: u16) -> Operand {
        Operand::Dice(Dice::new(num, face))
    }

    pub fn num(num: u16) -> Operand {
        Operand::Num(num)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Dice(d) => {
                write!(f, "{}", d)
            }
            Operand::Num(n) => {
                write!(f, "{}", n)
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

#[derive(Debug)]
pub struct Dice {
    pub num: DiceNum,
    pub face: u16,
    results: Vec<u16>,
    total: u64,
}

impl Dice {
    fn new(num: DiceNum, face: u16) -> Self {
        let rolls_num = match num {
            DiceNum::Num(n) => n,
            _ => 2,
        };
        let results: Vec<_> = (0..rolls_num)
            .map(|_| rand::thread_rng().gen_range(0..if face == 0 { 20 } else { face }) + 1)
            .collect();
        let results_clone = results.clone();
        let total = match num {
            DiceNum::Advantage => results_clone.into_iter().max().unwrap_or(0) as u64,
            DiceNum::Disadvantage => results_clone.into_iter().min().unwrap_or(0) as u64,
            DiceNum::Num(_) => {
                let mut sum = 0;
                results_clone.into_iter().for_each(|r| sum += r as u64);
                sum
            }
        };
        Self {
            num,
            face,
            results,
            total,
        }
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_str = if self.face == 0 {
            "⛧".to_owned()
        } else {
            self.face.to_string()
        };
        let face_int = if self.face == 0 { 20 } else { self.face };
        match self.num {
            DiceNum::Advantage => {
                write!(f, "`d{} with advantage`", face_str)?;
            }
            DiceNum::Disadvantage => {
                write!(f, "`d{} with disadvantage`", face_str)?;
            }
            DiceNum::Num(n) => {
                write!(f, "`{}d{}`", n, face_str)?;
            }
        };
        let mut roll_results = self
            .results
            .iter()
            .map(|roll| {
                if *roll == 1 || *roll == face_int {
                    format!("*{}*", roll)
                } else {
                    format!("{}", roll)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        if self.face == 0 {
            zalgofy(&mut roll_results);
        }
        write!(f, "\\[{}]", roll_results)
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
                write!(f, "{}", v)
            }
            Expression::Plus(a, b) => {
                write!(f, "{} + {}", a, b)
            }
            Expression::Minus(a, b) => {
                write!(f, "{} - {}", a, b)
            }
            Expression::Multiply(a, b) => {
                match **a {
                    Expression::Plus(_, _) | Expression::Minus(_, _) => {
                        write!(f, "({})", a)?;
                    }
                    _ => {
                        write!(f, "{}", a)?;
                    }
                }
                write!(f, " \\* ")?;
                match **b {
                    Expression::Plus(_, _) | Expression::Minus(_, _) => {
                        write!(f, "({})", b)?;
                    }
                    _ => {
                        write!(f, "{}", b)?;
                    }
                }
                Ok(())
            }
            Expression::Divide(a, b) => {
                match **a {
                    Expression::Plus(_, _) | Expression::Minus(_, _) => {
                        write!(f, "({})", a)?;
                    }
                    _ => {
                        write!(f, "{}", a)?;
                    }
                }
                write!(f, " ÷ ")?;
                match **b {
                    Expression::Plus(_, _) | Expression::Minus(_, _) => {
                        write!(f, "({})", b)?;
                    }
                    _ => {
                        write!(f, "{}", b)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::Value(Operand::dice(DiceNum::Num(1), 20))
    }
}

impl Expression {
    pub fn calc(&self) -> u64 {
        match self {
            Expression::Value(operand) => match operand {
                Operand::Dice(d) => d.total,
                Operand::Num(n) => *n as u64,
            },
            Expression::Plus(a, b) => a.calc() + b.calc(),
            Expression::Minus(a, b) => a.calc() - b.calc(),
            Expression::Multiply(a, b) => a.calc() * b.calc(),
            Expression::Divide(a, b) => a.calc() / b.calc(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            Some(comment) => {
                const CHARS_TO_ESCAPE: [char; 5] = ['\\', '`', '*', '_', '['];

                let mut escaped_comment = String::new();

                for c in comment.chars() {
                    if CHARS_TO_ESCAPE.contains(&c) {
                        escaped_comment.push('\\');
                    }
                    escaped_comment.push(c)
                }

                Self {
                    expression,
                    comment: Some(escaped_comment),
                }
            }
        }
    }
}

impl Default for RollLine {
    fn default() -> Self {
        Self {
            expression: Expression::default(),
            comment: None,
        }
    }
}

peg::parser! {
    grammar roll_parser() for str {

        rule _()
        = [' ' | '\n' | '\t']*

        rule space()
        = [' ' | '\n' | '\t']+

        rule num() -> u16
        = num:$(['0'..='9']+) {? num.parse().or(Err("The number is too big")) }

        rule dice_num() -> DiceNum
        = num:$(num() / "+" / "-") {? num.parse().or(Err("I don't have that many dices!")) }

        rule dice() -> Dice
        = num:dice_num()? ['d' | 'D' | 'к' | 'д'] face:num() { Dice::new(num.unwrap_or(DiceNum::Num(1)), face) }

        rule dice_operand() -> Operand
        = dice:dice() { Operand::Dice(dice) }

        rule num_operand() -> Operand
        = num:num() { Operand::Num(num) }

        pub rule operand() -> Operand
        = dice_operand() / num_operand()

        rule full_expression() -> Expression
        = precedence!{
            x:(@) "+" y:@ { Expression::Plus(Box::new(x), Box::new(y)) }
            x:(@) "-" y:@ { Expression::Minus(Box::new(x), Box::new(y)) }
            --
            x:(@) "*" y:@ { Expression::Multiply(Box::new(x), Box::new(y)) }
            x:(@) "÷" y:@ { Expression::Divide(Box::new(x), Box::new(y)) }
            --
            _ n:operand() _ { Expression::Value(n) }
            "(" _ e:full_expression() _ ")" { e }
        }

        rule short_adv() -> Expression
        = _ sign:$['+' | '-' ] _ {
            match sign {
                "+" => Expression::Value(Operand::dice(DiceNum::Advantage, 20)),
                "-" => Expression::Value(Operand::dice(DiceNum::Disadvantage, 20)),
                _ => unreachable!()
            }
        }

        rule short_bonus() -> Expression
        = _ sign:$['+' | '-' | '*' | '/' ] _ num:num() _ {
            match sign {
                "+" => Expression::Plus(
                        Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                        Box::new(Expression::Value(Operand::num(num)))
                    ),
                "-" => Expression::Minus(
                        Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                        Box::new(Expression::Value(Operand::num(num)))
                    ),
                "*" => Expression::Multiply(
                        Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                        Box::new(Expression::Value(Operand::num(num)))
                    ),
                "÷" => Expression::Divide(
                        Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                        Box::new(Expression::Value(Operand::num(num)))
                    ),
                _ => unreachable!()
            }
        }

        pub rule expression() -> Expression
        = full_expression() / short_bonus() / short_adv()

        rule comment() -> Option<String>
        = c:$((!expression() [_])+) {
            if c.is_empty() {
                None
            } else {
                const TRIM: [char; 4] = ['\\', ',', ';', '.'];
                let c = c.trim_matches(|c: char| c.is_whitespace() || TRIM.contains(&c) );
                Some(c.to_owned())
            }
        }

        rule expression_with_comment() -> RollLine
        = e:expression() c:comment()? {
            RollLine::new(e, c.flatten())
        }

        rule only_comment() -> RollLine
        = c:comment() {
            let expression = Expression::default();
            RollLine::new(expression, c)
        }

        rule roll_line() -> RollLine
        = expression_with_comment() / only_comment()

        rule nothing() -> Vec<RollLine>
        = ![_] {
            vec![RollLine::default()]
        }


        pub rule expressions() -> Vec<RollLine>
        = roll_line()+ / nothing()

    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse_operand() {
        assert_eq!(
            roll_parser::operand("1d20"),
            Ok(Operand::dice(DiceNum::Num(1), 20))
        );
        assert_eq!(roll_parser::operand("5"), Ok(Operand::num(5)));
        assert_eq!(
            roll_parser::operand("+d20"),
            Ok(Operand::dice(DiceNum::Advantage, 20))
        );
    }

    #[test]
    fn test_parse_expression() {
        assert_eq!(
            roll_parser::expression("1d20 + 5"),
            Ok(Expression::Plus(
                Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                Box::new(Expression::Value(Operand::Num(5)))
            ))
        );
        assert_eq!(
            roll_parser::expression("+d20"),
            Ok(Expression::Value(Operand::dice(DiceNum::Advantage, 20)))
        );
        assert_eq!(
            roll_parser::expression("d20"),
            Ok(Expression::Value(Operand::dice(DiceNum::Num(1), 20)))
        );
        assert_eq!(
            roll_parser::expression("d4"),
            Ok(Expression::Value(Operand::dice(DiceNum::Num(1), 4)))
        );
        assert_eq!(
            roll_parser::expression("d6+d4+3"),
            Ok(Expression::Plus(
                Box::new(Expression::Plus(
                    Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 6))),
                    Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 4)))
                )),
                Box::new(Expression::Value(Operand::Num(3)))
            ))
        );
        assert_eq!(
            roll_parser::expression("+d20+5"),
            Ok(Expression::Plus(
                Box::new(Expression::Value(Operand::dice(DiceNum::Advantage, 20))),
                Box::new(Expression::Value(Operand::Num(5)))
            ))
        );
        assert_eq!(
            roll_parser::expression("+"),
            Ok(Expression::Value(Operand::dice(DiceNum::Advantage, 20)))
        );
        assert_eq!(
            roll_parser::expression("+5"),
            Ok(Expression::Plus(
                Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                Box::new(Expression::Value(Operand::Num(5)))
            ))
        );
    }

    #[test]
    fn test_parse_expressions() {
        assert_eq!(
            roll_parser::expressions("1d20 + 5"),
            Ok(vec![RollLine {
                expression: Expression::Plus(
                    Box::new(Expression::Value(Operand::dice(DiceNum::Num(1), 20))),
                    Box::new(Expression::Value(Operand::Num(5)))
                ),
                comment: None,
            }])
        );

        assert_eq!(
            roll_parser::expressions("1d20 1d6"),
            Ok(vec![
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Num(1), 20)),
                    comment: None,
                },
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Num(1), 6)),
                    comment: None,
                }
            ])
        );

        assert_eq!(
            roll_parser::expressions("+ + +"),
            Ok(vec![
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Advantage, 20)),
                    comment: None,
                },
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Advantage, 20)),
                    comment: None,
                },
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Advantage, 20)),
                    comment: None,
                },
            ])
        );
    }

    #[test]
    fn test_display_expression() {
        let expr = roll_parser::expression("1d10").unwrap();
        print!("{}", expr);
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
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        let expr = expr.get(0).unwrap();
        assert_eq!(expr.comment, Some("to sneak the target".to_owned()));
    }

    #[test]
    fn test_mixed_comment() {
        let expr = roll_parser::expressions("d20 to sneak the target 2d6 damage");
        println!("{:?}", expr);
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert_eq!(
            expr,
            vec![
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Num(1), 20)),
                    comment: Some("to sneak the target".to_owned()),
                },
                RollLine {
                    expression: Expression::Value(Operand::dice(DiceNum::Num(2), 6)),
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
                expression: Expression::Value(Operand::dice(DiceNum::Num(1), 20)),
                comment: Some("to sneak the target".to_owned()),
            }]
        );
    }
}
