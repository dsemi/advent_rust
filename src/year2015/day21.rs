use crate::utils::parsers::*;
use derive_more::Add;
use itertools::Itertools;
use num::Integer;
use std::cmp::max;
use std::sync::LazyLock;

#[derive(Add, Clone, Copy)]
struct Equip {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Clone, Copy)]
struct Person {
    hitpoints: i32,
    equip: Equip,
}

#[rustfmt::skip]
const SHOP1: &[Equip] = &[
    Equip { cost: 8, damage: 4, armor: 0 },  // Dagger
    Equip { cost: 10, damage: 5, armor: 0 }, // Shortsword
    Equip { cost: 25, damage: 6, armor: 0 }, // Warhammer
    Equip { cost: 40, damage: 7, armor: 0 }, // Longsword
    Equip { cost: 74, damage: 8, armor: 0 }, // Greataxe
];
#[rustfmt::skip]
const SHOP2: &[Equip] = &[
    Equip { cost: 13, damage: 0, armor: 1 },  // Leather
    Equip { cost: 31, damage: 0, armor: 2 },  // Chainmail
    Equip { cost: 53, damage: 0, armor: 3 },  // Splintmail
    Equip { cost: 75, damage: 0, armor: 4 },  // Bandedmail
    Equip { cost: 102, damage: 0, armor: 5 }, // Platemail
];
#[rustfmt::skip]
const SHOP3: &[Equip] = &[
    Equip { cost: 25, damage: 1, armor: 0 },  // Damage +1
    Equip { cost: 50, damage: 2, armor: 0 },  // Damage +2
    Equip { cost: 100, damage: 3, armor: 0 }, // Damage +3
    Equip { cost: 20, damage: 0, armor: 1 },  // Defense +1
    Equip { cost: 40, damage: 0, armor: 2 },  // Defense +2
    Equip { cost: 80, damage: 0, armor: 3 },  // Defense +3
    Equip { cost: 0, damage: 0, armor: 0 },   // None
];
static ALL_EQUIP_COMBOS: LazyLock<Vec<Person>> = LazyLock::new(|| {
    let mut v = Vec::new();
    for &weapon in SHOP1 {
        for &armor in SHOP2 {
            for rings in SHOP3.iter().combinations(2) {
                v.push(person(weapon + armor + *rings[0] + *rings[1]));
            }
            v.push(person(weapon + armor));
        }
    }
    v
});

fn is_winning(boss: Person, player: Person) -> bool {
    fn ttd(p1: Person, p2: Person) -> i32 {
        Integer::div_ceil(&p1.hitpoints, &max(1, p2.equip.damage - p1.equip.armor))
    }
    ttd(player, boss) >= ttd(boss, player)
}

fn person(equip: Equip) -> Person {
    Person {
        hitpoints: 100,
        equip,
    }
}

fn parse_boss(i: &mut &str) -> PResult<Person> {
    let hitpoints = preceded("Hit Points: ", i32).parse_next(i)?;
    let damage = preceded("\nDamage: ", i32).parse_next(i)?;
    let armor = preceded("\nArmor: ", i32).parse_next(i)?;
    Ok(Person {
        hitpoints,
        equip: Equip {
            cost: 0,
            damage,
            armor,
        },
    })
}

pub fn part1(input: &str) -> Option<i32> {
    let boss = parse_boss.read(input);
    ALL_EQUIP_COMBOS
        .iter()
        .filter(|&&p| is_winning(boss, p))
        .map(|p| p.equip.cost)
        .min()
}

pub fn part2(input: &str) -> Option<i32> {
    let boss = parse_boss.read(input);
    ALL_EQUIP_COMBOS
        .iter()
        .filter(|&&p| !is_winning(boss, p))
        .map(|p| p.equip.cost)
        .max()
}
