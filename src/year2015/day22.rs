use crate::utils::parsers::*;
use crate::utils::*;
use std::cmp::max;

struct Spell {
    cost: i32,
    effect: fn(&mut Game),
    active: fn(&Game) -> bool,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Game {
    player_health: i32,
    player_mana: i32,
    player_armor: i32,
    boss_health: i32,
    boss_damage: i32,
    shield_turns: i32,
    poison_turns: i32,
    recharge_turns: i32,
}

const SPELLS: &[Spell] = &[
    Spell {
        // Magic Missile
        cost: 53,
        effect: |state| state.boss_health -= 4,
        active: |_| false,
    },
    Spell {
        // Drain
        cost: 73,
        effect: |state| {
            state.player_health += 2;
            state.boss_health -= 2;
        },
        active: |_| false,
    },
    Spell {
        // Shield
        cost: 113,
        effect: |state| {
            state.player_armor += 7;
            state.shield_turns = 6;
        },
        active: |state| state.shield_turns > 0,
    },
    Spell {
        // Poison
        cost: 173,
        effect: |state| state.poison_turns = 6,
        active: |state| state.poison_turns > 0,
    },
    Spell {
        // Recharge
        cost: 229,
        effect: |state| state.recharge_turns = 5,
        active: |state| state.recharge_turns > 0,
    },
];

fn apply_effects(state: &mut Game) {
    if state.shield_turns > 0 {
        if state.shield_turns == 1 {
            state.player_armor -= 7;
        }
        state.shield_turns -= 1;
    }
    if state.poison_turns > 0 {
        state.boss_health -= 3;
        state.poison_turns -= 1;
    }
    if state.recharge_turns > 0 {
        state.player_mana += 101;
        state.recharge_turns -= 1;
    }
}

fn parse_boss(i: &mut &str) -> PResult<Game> {
    let boss_health = preceded("Hit Points: ", i32).parse_next(i)?;
    let boss_damage = preceded("\nDamage: ", i32).parse_next(i)?;
    Ok(Game {
        player_health: 50,
        player_mana: 500,
        player_armor: 0,
        boss_health,
        boss_damage,
        shield_turns: 0,
        poison_turns: 0,
        recharge_turns: 0,
    })
}

fn neighbors(s: &Game, hard: bool) -> Vec<(usize, Game)> {
    let mut state = s.clone();
    if hard {
        state.player_health -= 1;
        if state.player_health <= 0 {
            return vec![];
        }
    }
    apply_effects(&mut state);
    if state.boss_health <= 0 {
        // Don't bother checking more since we've won already.
        return vec![(0, state)];
    }
    let mut neighbs = Vec::new();
    for spell in SPELLS {
        if state.player_mana >= spell.cost && !(spell.active)(&state) {
            let mut new_state = state.clone();
            new_state.player_mana -= spell.cost;
            (spell.effect)(&mut new_state);
            apply_effects(&mut new_state);
            new_state.player_health -= max(1, new_state.boss_damage - new_state.player_armor);
            if new_state.boss_health <= 0 || new_state.player_health > 0 {
                neighbs.push((spell.cost as usize, new_state));
            }
        }
    }
    neighbs
}

pub fn part1(input: &str) -> Option<usize> {
    dijkstra(parse_boss.read(input), |s| neighbors(s, false))
        .find_map(|(d, s)| (s.boss_health <= 0).then_some(d))
}

pub fn part2(input: &str) -> Option<usize> {
    dijkstra(parse_boss.read(input), |s| neighbors(s, true))
        .find_map(|(d, s)| (s.boss_health <= 0).then_some(d))
}
