use http_service::Body;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use tide::{
    http::{headers, StatusCode},
    IntoResponse, Response,
};

// All the possible D&D dice
const DICE_VALUES: [i32; 7] = [4, 6, 8, 10, 12, 20, 100];

#[derive(Debug, Deserialize, PartialEq, Serialize)]
/// Instructions for a roll
pub struct RollInstruction {
    /// Number of dice to roll
    pub num: i32,
    /// Number of sides on the dice
    pub die: i32,
    /// Additional modifier to add to the roll
    pub modifier: i32,
}

impl fmt::Display for RollInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.num, self.die)?;
        if self.modifier != 0 {
            write!(
                f,
                " {} {}",
                if self.modifier < 0 { "-" } else { "+" },
                self.modifier.abs()
            )?;
        }
        Ok(())
    }
}

impl From<RollInstruction> for String {
    #[must_use]
    fn from(instruction: RollInstruction) -> Self {
        format!("{}", instruction)
    }
}

#[derive(Serialize, Debug)]
pub struct RollError {
    pub message: String,
}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for RollError {
    fn into_response(self) -> Response {
        Response::new(StatusCode::BadRequest)
            .body(Body::from(serde_json::to_vec(&self).unwrap()))
            .set_header(headers::CONTENT_TYPE, "application/json")
    }
}

#[derive(Serialize, Debug)]
/// Result of a roll
pub struct RollResult {
    /// The instruction passed in to roll the dice
    pub instruction: String,
    /// The results of all rolls made
    pub rolls: Vec<i32>,
    /// The total value of the entire roll
    pub total: i32,
}

/// # Errors
///
/// Will return `RollError` if format is invalid
pub fn parse_roll(cmd: &str) -> Result<RollInstruction, RollError> {
    let re = Regex::new(r"(?P<num>\d+)d(?P<dice>\d+)(\s*\+\s*(?P<modifier>\d+))?").unwrap();
    if re.is_match(cmd) {
        Ok(re
            .captures_iter(cmd)
            .map(|cap| RollInstruction {
                num: cap["num"].parse().unwrap(),
                die: cap["dice"].parse().unwrap(),
                modifier: match cap.name("modifier") {
                    Some(m) => m.as_str().parse().unwrap(),
                    None => 0,
                },
            })
            .take(1)
            .next()
            .unwrap())
    } else {
        Err(RollError {
            message: String::from("Invalid format. Try again with something like 1d20 or 3d6."),
        })
    }
}

fn gen_roll(rng: &mut impl Rng, die: i32) -> i32 {
    rng.gen_range(1, die + 1)
}

/// # Errors
///
/// Will return `RollError` if instruction is invalid
pub fn roll(rng: &mut impl Rng, instruction: RollInstruction) -> Result<RollResult, RollError> {
    // let mut rng = Pcg64::from_entropy();
    let mut total = 0;
    let mut rolls = Vec::new();
    if !DICE_VALUES.iter().any(|d| d == &instruction.die) {
        return Err(RollError {
            message: format!(
                "Not a valid die. Try one of {}",
                DICE_VALUES
                    .iter()
                    .map(|d| format!("d{}", d))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        });
    } else if instruction.num < 1 {
        return Err(RollError {
            message: String::from("You have to roll something!"),
        });
    } else if instruction.num > 99 {
        return Err(RollError {
            message: String::from(
                "Are you a god in this game?! Roll a more reasonable number of dice!",
            ),
        });
    }
    for _ in 0..instruction.num {
        let roll = gen_roll(rng, instruction.die);
        total += roll;
        rolls.push(roll);
    }
    total += instruction.modifier;

    Ok(RollResult {
        instruction: instruction.into(),
        rolls,
        total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;
    use std::collections::HashMap;

    #[test]
    fn test_parse_roll_single_dice() {
        let roll = parse_roll("1d8").unwrap();
        assert_eq!(
            roll,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: 0
            }
        );
    }

    #[test]
    fn test_parse_roll_multiple_dice() {
        let roll = parse_roll("3d6").unwrap();
        assert_eq!(
            roll,
            RollInstruction {
                num: 3,
                die: 6,
                modifier: 0
            }
        );
    }

    #[test]
    fn test_parse_roll_modifier() {
        let roll = parse_roll("1d8 + 3").unwrap();
        assert_eq!(
            roll,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: 3
            }
        );
    }

    #[test]
    fn test_parse_roll_modifier_spacing() {
        let roll1 = parse_roll("1d8 + 3").unwrap();
        let roll2 = parse_roll("1d8+ 3").unwrap();
        let roll3 = parse_roll("1d8 +3").unwrap();
        let roll4 = parse_roll("1d8+3").unwrap();
        assert_eq!(roll1, roll2);
        assert_eq!(roll1, roll3);
        assert_eq!(roll1, roll4);
    }

    #[test]
    #[should_panic]
    fn test_parse_roll_fail() {
        parse_roll("3e6").unwrap();
    }

    #[test]
    fn test_gen_roll() {
        let mut rng = rand::thread_rng();

        for d in &DICE_VALUES {
            let mut occurrences: HashMap<i32, i32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll = gen_roll(&mut rng, *d);
                let count = occurrences.entry(roll).or_insert(0);
                *count += 1;
            }

            // Assert that all values for 1 through d have at least one roll
            for i in 1..=*d {
                assert!(occurrences[&i] > 0)
            }
        }
    }

    #[test]
    fn test_roll_single_dice() {
        let mut rng = Pcg64::from_entropy();
        let roll = roll(
            &mut rng,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: 0,
            },
        )
        .unwrap();
        assert!(roll.total >= 1);
        assert!(roll.total <= 8);
    }

    #[test]
    fn test_roll_multiple_dice() {
        let mut rng = Pcg64::from_entropy();
        let roll = roll(
            &mut rng,
            RollInstruction {
                num: 3,
                die: 6,
                modifier: 0,
            },
        )
        .unwrap();
        assert!(roll.total >= 3);
        assert!(roll.total <= 18);
    }

    #[test]
    fn test_roll_multiple_dice_modifier() {
        let mut rng = Pcg64::from_entropy();
        let roll = roll(
            &mut rng,
            RollInstruction {
                num: 3,
                die: 6,
                modifier: 3,
            },
        )
        .unwrap();
        assert!(roll.total >= 6);
        assert!(roll.total <= 21);
    }

    #[test]
    #[should_panic]
    fn test_roll_invalid_dice() {
        let mut rng = Pcg64::from_entropy();
        roll(
            &mut rng,
            RollInstruction {
                num: 0,
                die: 9,
                modifier: 0,
            },
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_roll_too_few() {
        let mut rng = Pcg64::from_entropy();
        roll(
            &mut rng,
            RollInstruction {
                num: 0,
                die: 8,
                modifier: 0,
            },
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_roll_too_many() {
        let mut rng = Pcg64::from_entropy();
        roll(
            &mut rng,
            RollInstruction {
                num: 100,
                die: 8,
                modifier: 0,
            },
        )
        .unwrap();
    }
}
