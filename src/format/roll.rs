use std::fmt::Display;
use std::str::FromStr;

use rand::prelude::*;
use regex::Regex;
use thiserror::Error;

use crate::format::utils::zalgofy;

#[derive(Error, Debug)]
pub enum DieFormatError {
    #[error("Formatting error ocurred: {0}")]
    Other(String),
    #[error("I don't have so many dices!")]
    TooManyDices,
    #[error("I don't have a suitable dice tray for that!")]
    TooManyRolls,
    #[error("I don't have such a large die!")]
    TooLargeDie,
}

#[derive(Debug)]
pub enum RollMode {
    ADV,
    DADV,
    NORMAL,
}

#[derive(Debug)]
pub struct Roll {
    pub die: String,
    pub roll_results: Vec<i32>,
    pub total: i32,
    pub mode: RollMode,
    pub num: i32,
    pub face: i32,
    pub zalgo: bool,
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let roll_results = self.roll_results.iter().map(|roll| {
            if *roll == 1 || *roll == self.face {
                format!("*{}*", roll)
            } else {
                format!("{}", roll)
            }
        });

        let mut die = self.die.clone();
        let mut total = format!("{}", self.total);
        let mut roll_results_str: String;
        if self.zalgo {
            roll_results_str = roll_results.take(6).collect::<Vec<_>>().join(", ");

            zalgofy(&mut die);
            zalgofy(&mut roll_results_str);
            zalgofy(&mut total);
        } else {
            roll_results_str = roll_results.collect::<Vec<_>>().join(", ");
        }

        write!(f, "`{}:` \\[{}] = *{}*", die, roll_results_str, total)
    }
}

pub fn roll_dice(msg: &str) -> Result<String, DieFormatError> {
    let response = roll_results(msg)?
        .iter()
        .map(|roll| roll.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if response.len() == 0 {
        warn!("Cannot parse: {}", msg);
        Ok("Err, sorry, I can't roll that. Maybe you need some /help ?".to_owned())
    } else {
        Ok(response)
    }
}

pub fn roll_results(msg: &str) -> Result<Vec<Roll>, DieFormatError> {
    lazy_static! {
        static ref DICE_REGEX: Regex = Regex::new(r"(?P<num>\+|\-|\d+)?(?:(?:d|к|д)(?P<face>\d+))?\s*(?:(?P<bonus_sign>\+|\-|\*|/)\s*(?P<bonus_value>\d+))?").unwrap();
    }

    const MAX_ROLLS: usize = 20;
    const MAX_FACES: i32 = 10000;
    const MAX_NUM: i32 = 100;

    let mut result = Vec::new();

    // Small trick to get type hints work for lazy static
    let dice_regex: &Regex = &*DICE_REGEX;
    let iter = dice_regex.captures_iter(msg).enumerate();

    if iter.size_hint().0 > MAX_ROLLS {
        return Err(DieFormatError::TooManyRolls);
    }

    for (i, cap) in iter {
        if i > MAX_ROLLS {
            return Err(DieFormatError::TooManyRolls);
        }

        if msg != ""
            && cap
                .name("num")
                .or(cap.name("face"))
                .or(cap.name("bonus_sign"))
                .or(cap.name("bonus_value"))
                == None
        {
            continue;
        }

        let num = cap.name("num").map_or("1", |m| m.as_str());
        let (face, zalgo): (i32, bool) = {
            let face = cap
                .name("face")
                .map(|m| FromStr::from_str(m.as_str()).ok())
                .flatten()
                .unwrap_or(20);
            if face == 0 {
                (666, true)
            } else {
                (face, false)
            }
        };

        if face > MAX_FACES {
            return Err(DieFormatError::TooLargeDie);
        }

        let bonus_sign = cap.name("bonus_sign").map(|m| m.as_str());
        let bonus_value: Option<i32> = cap
            .name("bonus_value")
            .map(|m| FromStr::from_str(m.as_str()).ok())
            .flatten();

        let (mode, capacity) = match num {
            "+" => (RollMode::ADV, 2),
            "-" => (RollMode::DADV, 2),
            _ => (
                RollMode::NORMAL,
                FromStr::from_str(num).map_err(|_| DieFormatError::TooManyDices)?,
            ),
        };

        if capacity > MAX_NUM {
            return Err(DieFormatError::TooManyDices);
        }

        let roll_results: Vec<i32> = (0..capacity)
            .map(|_| rand::thread_rng().gen_range(0, face) + 1)
            .collect();

        let mut total: i32 = match mode {
            RollMode::ADV => *roll_results.iter().max().unwrap_or(&0),
            RollMode::DADV => *roll_results.iter().min().unwrap_or(&0),
            RollMode::NORMAL => roll_results.iter().sum(),
        };

        let mut die = match mode {
            RollMode::ADV => format!("d{} with advantage", face),
            RollMode::DADV => format!("d{} with disadvantage", face),
            RollMode::NORMAL => format!("{}d{}", num, face),
        };

        if let (Some(bonus_sign), Some(bonus_value)) = (bonus_sign, bonus_value) {
            die.push_str(&format!(" {} {}", bonus_sign, bonus_value));
            match bonus_sign {
                "+" => total += bonus_value,
                "-" => total -= bonus_value,
                "*" => total *= bonus_value,
                "/" => total /= bonus_value,
                other => {
                    let err = DieFormatError::Other(format!(
                        "Cannot parse roll expression: unknown symbol {}",
                        other
                    ));
                    error!("{} in message: {}", err, msg);
                    return Err(err);
                }
            }
        }

        result.push(Roll {
            die,
            total,
            roll_results,
            mode,
            face,
            num: capacity,
            zalgo,
        })
    }

    Ok(result)
}
