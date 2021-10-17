use std::cmp::{max, min};

struct Spell {
    cost: i32,
    effect: fn(&mut Game),
    active: fn(&Game) -> bool,
}

#[derive(Clone)]
struct Game {
    player_health: i32,
    player_mana: i32,
    player_armor: i32,
    spent_mana: i32,
    boss_health: i32,
    boss_damage: i32,
    shield_turns: i32,
    poison_turns: i32,
    recharge_turns: i32,
}

lazy_static! {
    static ref SPELLS: Vec<Spell> = vec![
        Spell { // Magic Missile
            cost: 53,
            effect: |state| state.boss_health -= 4,
            active: |_| false,
        },
        Spell { // Drain
            cost: 73,
            effect: |state| {
                state.player_health += 2;
                state.boss_health -= 2;
            },
            active: |_| false,
        },
        Spell { // Shield
            cost: 113,
            effect: |state| {
                state.player_armor += 7;
                state.shield_turns = 6;
            },
            active: |state| state.shield_turns > 0,
        },
        Spell { // Poison
            cost: 173,
            effect: |state| state.poison_turns = 6,
            active: |state| state.poison_turns > 0,
        },
        Spell { // Recharge
            cost: 229,
            effect: |state| state.recharge_turns = 5,
            active: |state| state.recharge_turns > 0,
        },
    ];
}

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

fn parse_boss(input: &str) -> Game {
    let v: Vec<i32> = input
        .lines()
        .map(|x| x.split(": ").last().unwrap().parse().unwrap())
        .collect();
    Game {
        player_health: 50,
        player_mana: 500,
        player_armor: 0,
        spent_mana: 0,
        boss_health: v[0],
        boss_damage: v[1],
        shield_turns: 0,
        poison_turns: 0,
        recharge_turns: 0,
    }
}

fn min_cost_to_win(s: Game, hard: bool) -> Option<i32> {
    let mut states = vec![s];
    let mut result = None;
    while let Some(mut state) = states.pop() {
        if hard {
            state.player_health -= 1;
            if state.player_health <= 0 {
                continue;
            }
        }
        apply_effects(&mut state);
        if state.boss_health <= 0 {
            result = Some(result.map_or(state.spent_mana, |v| min(v, state.spent_mana)));
            continue;
        }
        for spell in SPELLS.iter() {
            if state.player_mana >= spell.cost
                && (result.is_none() || state.spent_mana + spell.cost < result.unwrap())
                && !(spell.active)(&state)
            {
                let mut new_state = state.clone();
                new_state.player_mana -= spell.cost;
                new_state.spent_mana += spell.cost;
                (spell.effect)(&mut new_state);
                apply_effects(&mut new_state);
                if new_state.boss_health <= 0 {
                    result = Some(new_state.spent_mana);
                    continue;
                }
                new_state.player_health -= max(1, new_state.boss_damage - new_state.player_armor);
                if new_state.player_health > 0 {
                    states.push(new_state);
                }
            }
        }
    }
    result
}

pub fn part1(input: &str) -> Option<i32> {
    min_cost_to_win(parse_boss(input), false)
}

pub fn part2(input: &str) -> Option<i32> {
    min_cost_to_win(parse_boss(input), true)
}
