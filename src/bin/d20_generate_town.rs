#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
use async_std::io;
use d20::{
    dice_roller::{self, RollInstruction},
    rng_pool,
};
use rand::{
    distributions::{Distribution, Standard},
    seq::IteratorRandom,
    Rng,
};
use std::convert::TryInto;
use std::iter::{repeat_with, FromIterator};

const FAMILY_NAMES: &[&str] = &[
    "Alastroi",
    "Atonovich",
    "Antonova",
    "Barthos",
    "Belasco",
    "Cantemir",
    "Dargovich",
    "Dargova",
    "Diavolov",
    "Diminski",
    "Dilisnya",
    "Drazkoi",
    "Garvinski",
    "Grejenko",
    "Groza",
    "Grygorovich",
    "Grygorova",
    "Ivanovich",
    "Ivanova",
    "Janek",
    "Karushkin",
    "Konstantinovich",
    "Konstantinova",
    "Krezkov",
    "Krezcova",
    "Krykski",
    "Lansten",
    "Lazarescu",
    "Lukresh",
    "Lipsiege",
    "Martikov",
    "Marticova",
    "Mironovich",
    "Mironovna",
    "Moldovar",
    "Nikolovich",
    "Nikolova",
    "Nimirovich",
    "Nimirova",
    "Oronovich",
    "Oronova",
    "Petrovich",
    "Petrovna",
    "Polensky",
    "Radovich",
    "Radova",
    "Rilsky",
    "Stefanovich",
    "Stefanova",
    "Strazni",
    "Swilovich",
    "Swilova",
    "Taltos",
    "Targolov",
    "Targolova",
    "Tyminski",
    "Ulbrek",
    "Ulrich",
    "Vadu",
    "Voltanescu",
    "Zalenski",
    "Zalken",
];
const FEMALE_NAMES: &[&str] = &[
    "Alana",
    "Clavdia",
    "Danya",
    "Dezdrelda",
    "Diavola",
    "Dorina",
    "Drasha",
    "Drilvia",
    "Elisabeta",
    "Fatima",
    "Grilsha",
    "Isabella",
    "Ivana",
    "Jarzinka",
    "Kala",
    "Katerina",
    "Kereza",
    "Korina",
    "Lavinia",
    "Magda",
    "Marta",
    "Mathilda",
    "Minodora",
    "Mirabel",
    "Miruna",
    "Nimira",
    "Nyanka",
    "Olivenka",
    "Ruxandra",
    "Sorina",
    "Tereska",
    "Valentina",
    "Vasha",
    "Victoria",
    "Wensencia",
    "Zondra",
];
const MALE_NAMES: &[&str] = &[
    "Alek",
    "Andrej",
    "Anton",
    "Balthazar",
    "Bogan",
    "Boris",
    "Dargos",
    "Darzin",
    "Dragomir",
    "Emeric",
    "Falkon",
    "Frederick",
    "Franz",
    "Gargosh",
    "Gorek",
    "Grygori",
    "Hans",
    "Harkus",
    "Ivan",
    "Jirko",
    "Kobal",
    "Korga",
    "Krystofor",
    "Lazlo",
    "Livius",
    "Marek",
    "Miroslav",
    "Nikolaj",
    "Nimir",
    "oleg",
    "Radovan",
    "Radu",
    "Seraz",
    "Sergei",
    "Stefan",
    "Tural",
    "Valentin",
    "Vasily",
    "Vladislav",
    "Walter",
    "Yesper",
    "Zsolt",
];

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
enum Age {
    Adult,
    Child,
}
#[derive(Debug)]
enum Gender {
    Female,
    Male,
}
impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0, 2) {
            0 => Gender::Female,
            _ => Gender::Male,
        }
    }
}

#[derive(Debug)]
struct Villager {
    age: Age,
    gender: Gender,
    hp: i32,
    name: &'static str,
}
impl Villager {
    fn new(rng: &mut impl Rng, age: Age) -> Self {
        let roll = dice_roller::roll(
            rng,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: 0,
            },
        )
        .unwrap();
        let gender: Gender = rng.gen();
        let hp = match age {
            Age::Adult => roll.total,
            Age::Child => 1,
        };
        let name = match gender {
            Gender::Female => FEMALE_NAMES.iter().choose(rng).unwrap(),
            Gender::Male => MALE_NAMES.iter().choose(rng).unwrap(),
        };
        Self {
            age,
            gender,
            hp,
            name,
        }
    }
}
#[derive(Debug)]
struct Villagers {
    family: Vec<Villager>,
    family_name: &'static str,
}
impl Villagers {
    fn new(rng: &mut impl Rng) -> Self {
        let num_adults = dice_roller::roll(
            rng,
            RollInstruction {
                num: 1,
                die: 4,
                modifier: 0,
            },
        )
        .unwrap();
        let num_children = dice_roller::roll(
            rng,
            RollInstruction {
                num: 1,
                die: 8,
                modifier: -1,
            },
        )
        .unwrap();
        let mut family = Vec::new();
        for _ in 0..num_adults.total {
            family.push(Villager::new(rng, Age::Adult));
        }
        for _ in 0..num_children.total {
            family.push(Villager::new(rng, Age::Child));
        }
        Self {
            family,
            family_name: FAMILY_NAMES.iter().choose(rng).unwrap(),
        }
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

    println!("{:#?}", occupants);
}

fn main() -> io::Result<()> {
    let pool = rng_pool();
    let mut rng = pool.get().unwrap();

    house(&mut *rng);

    Ok(())
}
