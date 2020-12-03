use itertools::Itertools;
use std::cmp::max;
use std::ops::Add;

#[derive(Clone, Copy)]
struct Equip {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Add<Equip> for Equip {
    type Output = Equip;

    fn add(self, rhs: Equip) -> Equip {
        Equip {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

#[derive(Clone, Copy)]
struct Person {
    hitpoints: i32,
    equip: Equip,
}

lazy_static! {
    static ref SHOP1: Vec<Equip> = vec![
        Equip { cost: 8, damage: 4, armor: 0 }, // Dagger
        Equip { cost: 10, damage: 5, armor: 0 }, // Shortsword
        Equip { cost: 25, damage: 6, armor: 0 }, // Warhammer
        Equip { cost: 40, damage: 7, armor: 0 }, // Longsword
        Equip { cost: 74, damage: 8, armor: 0 }, // Greataxe
    ];
    static ref SHOP2: Vec<Equip> = vec![
        Equip { cost: 13, damage: 0, armor: 1 }, // Leather
        Equip { cost: 31, damage: 0, armor: 2 }, // Chainmail
        Equip { cost: 53, damage: 0, armor: 3 }, // Splintmail
        Equip { cost: 75, damage: 0, armor: 4 }, // Bandedmail
        Equip { cost: 102, damage: 0, armor: 5 }, // Platemail
    ];
    static ref SHOP3: Vec<Equip> = vec![
        Equip { cost: 25, damage: 1, armor: 0 }, // Damage +1
        Equip { cost: 50, damage: 2, armor: 0 }, // Damage +2
        Equip { cost: 100, damage: 3, armor: 0 }, // Damage +3
        Equip { cost: 20, damage: 0, armor: 1 }, // Defense +1
        Equip { cost: 40, damage: 0, armor: 2 }, // Defense +2
        Equip { cost: 80, damage: 0, armor: 3 }, // Defense +3
        Equip { cost: 0, damage: 0, armor: 0 }, // None
    ];
    static ref ALL_EQUIP_COMBOS: Vec<Person> = {
        let mut v = Vec::new();
        for &weapon in SHOP1.iter() {
            for &armor in SHOP2.iter() {
                for combo in SHOP3.iter().combinations(2) {
                    for rings in vec![*combo[0] + *combo[1], *combo[1]].into_iter() {
                        v.push(person(weapon + armor + rings));
                    }
                }
            }
        }
        v
    };
}

fn is_winning(boss: Person, player: Person) -> bool {
    fn ttd(p1: Person, p2: Person) -> i32 {
        let q = p1.hitpoints / max(1, p2.equip.damage - p1.equip.armor);
        if p1.hitpoints % max(1, p2.equip.damage - p1.equip.armor) == 0 {
            q
        } else {
            q + 1
        }
    }
    ttd(player, boss) >= ttd(boss, player)
}

fn person(equip: Equip) -> Person {
    Person {
        hitpoints: 100,
        equip: equip,
    }
}

fn parse_boss(input: &str) -> Person {
    let v: Vec<i32> = input
        .lines()
        .map(|line| line.split(" ").last().unwrap().parse().unwrap())
        .collect();
    Person {
        hitpoints: v[0],
        equip: Equip {
            cost: 0,
            damage: v[1],
            armor: v[2],
        },
    }
}

pub fn part1(input: &str) -> Option<i32> {
    let boss = parse_boss(input);
    ALL_EQUIP_COMBOS
        .iter()
        .filter(|&&p| is_winning(boss, p))
        .map(|p| p.equip.cost)
        .min()
}

pub fn part2(input: &str) -> Option<i32> {
    let boss = parse_boss(input);
    ALL_EQUIP_COMBOS
        .iter()
        .filter(|&&p| !is_winning(boss, p))
        .map(|p| p.equip.cost)
        .max()
}
