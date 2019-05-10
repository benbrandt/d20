use log::info;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RollInstruction {
    pub num: i32,
    pub die: i32,
    pub modifier: i32,
}

#[derive(Serialize, Debug)]
pub struct RollError {
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct DiceResult {
    pub die: i32,
    pub value: i32,
}

#[derive(Serialize, Debug)]
pub struct RollResult {
    pub instruction: RollInstruction,
    pub rolls: Vec<DiceResult>,
    pub total: i32,
}

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

fn gen_roll(rng: &mut ThreadRng, die: i32) -> DiceResult {
    let roll = rng.gen_range(1, die + 1);
    info!("Die: {}, Roll: {}", die, roll);
    DiceResult { die, value: roll }
}

pub fn roll(instruction: RollInstruction) -> Result<RollResult, RollError> {
    let mut rng = rand::thread_rng();
    let mut total = 0;
    let mut rolls = Vec::new();
    if instruction.num < 1 {
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
        let roll = gen_roll(&mut rng, instruction.die);
        total += roll.value;
        rolls.push(roll);
    }
    total += instruction.modifier;

    Ok(RollResult {
        instruction,
        rolls,
        total,
    })
}

#[cfg(test)]
mod tests {
    // All the possible D&D dice
    const DICE_VALUES: [i32; 7] = [4, 6, 8, 10, 12, 20, 100];

    use super::*;
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

        for d in DICE_VALUES.iter() {
            let mut occurrences: HashMap<i32, i32> = HashMap::new();
            // Try and get a sample that will have an occurrence for every value
            for _ in 0..d * d {
                let roll = gen_roll(&mut rng, *d);
                let count = occurrences.entry(roll.value).or_insert(0);
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
        let roll = roll(RollInstruction {
            num: 1,
            die: 8,
            modifier: 0,
        })
        .unwrap();
        assert!(roll.total >= 1);
        assert!(roll.total <= 8);
    }

    #[test]
    fn test_roll_multiple_dice() {
        let roll = roll(RollInstruction {
            num: 3,
            die: 6,
            modifier: 0,
        })
        .unwrap();
        assert!(roll.total >= 3);
        assert!(roll.total <= 18);
    }

    #[test]
    fn test_roll_multiple_dice_modifier() {
        let roll = roll(RollInstruction {
            num: 3,
            die: 6,
            modifier: 3,
        })
        .unwrap();
        assert!(roll.total >= 6);
        assert!(roll.total <= 21);
    }

    #[test]
    #[should_panic]
    fn test_roll_too_few() {
        roll(RollInstruction {
            num: 0,
            die: 8,
            modifier: 0,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_roll_too_many() {
        roll(RollInstruction {
            num: 100,
            die: 8,
            modifier: 0,
        })
        .unwrap();
    }
}
