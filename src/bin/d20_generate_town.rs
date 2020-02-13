#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
use async_std::io;
use d20::{
    dice_roller::{self, RollInstruction},
    rng_pool,
};
use rand::Rng;
use std::convert::TryInto;
use std::iter::{repeat_with, FromIterator};

#[derive(Debug)]
struct SwarmOfRats(i32);
impl SwarmOfRats {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 7,
                die: 8,
                modifier: -7,
            },
        )
        .unwrap();
        Self(roll.total)
    }
}
#[derive(Debug)]
struct SwarmsOfRats(Vec<SwarmOfRats>);
impl SwarmsOfRats {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 2,
                die: 4,
                modifier: 0,
            },
        )
        .unwrap();
        let swarms = repeat_with(|| SwarmOfRats::new(rng)).take(roll.total.try_into().unwrap());

        Self(Vec::from_iter(swarms))
    }
}
#[derive(Debug)]
struct Villager(i32);
impl Villager {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: 0,
            },
        )
        .unwrap();
        Self(roll.total)
    }
}
#[derive(Debug)]
struct Villagers(Vec<Villager>);
impl Villagers {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 1,
                die: 4,
                modifier: 0,
            },
        )
        .unwrap();
        let adults = repeat_with(|| Villager::new(rng)).take(roll.total.try_into().unwrap());

        Self(Vec::from_iter(adults))
    }
}
#[derive(Debug)]
struct Zombie(i32);
impl Zombie {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 4,
                die: 8,
                modifier: 12,
            },
        )
        .unwrap();
        Self(roll.total)
    }
}
#[derive(Debug)]
struct Zombies(Vec<Zombie>);
impl Zombies {
    fn new(rng: &mut impl Rng) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 2,
                die: 4,
                modifier: 0,
            },
        )
        .unwrap();
        let zombies = repeat_with(|| Zombie::new(rng)).take(roll.total.try_into().unwrap());

        Self(Vec::from_iter(zombies))
    }
}

#[derive(Debug)]
enum Occupants {
    Empty,
    SwarmsOfRats(SwarmsOfRats),
    Villagers(Villagers),
    Zombies(Zombies),
}

fn house(rng: &mut impl Rng) {
    let roll = dice_roller::roll(
        rng,
        RollInstruction {
            num: 1,
            die: 20,
            modifier: 0,
        },
    )
    .unwrap();

    let occupants = match roll.total {
        4..=8 => Occupants::SwarmsOfRats(SwarmsOfRats::new(rng)),
        9..=16 => Occupants::Villagers(Villagers::new(rng)),
        17..=20 => Occupants::Zombies(Zombies::new(rng)),
        _ => Occupants::Empty,
    };

    println!("{:?}", occupants);
}

fn main() -> io::Result<()> {
    let pool = rng_pool();
    let mut rng = pool.get().unwrap();

    house(&mut *rng);

    Ok(())
}
