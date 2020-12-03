use std::cmp::max;
use Id::*;

#[derive(Clone, PartialEq)]
enum Id {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

struct Spell {
    id: Id,
    cost: i32,
    func: fn(&mut Game),
}

#[derive(Clone)]
struct Game {
    player_health: i32,
    player_mana: i32,
    player_armor: i32,
    boss_health: i32,
    boss_damage: i32,
    player_turn: bool,
    hard: bool,
    effects: Vec<(Id, Vec<fn(&mut Game)>)>,
}

lazy_static! {
    static ref SPELLS: Vec<Spell> = vec![
        Spell {
            id: MagicMissile,
            cost: 53,
            func: |state| state.boss_health -= 4,
        },
        Spell {
            id: Drain,
            cost: 73,
            func: |state| {
                state.player_health += 2;
                state.boss_health -= 2;
            },
        },
        Spell {
            id: Shield,
            cost: 113,
            func: |state| {
                state.effects.push((
                    Shield,
                    vec![
                        |state| state.player_armor += 7,
                        |_| {},
                        |_| {},
                        |_| {},
                        |_| {},
                        |state| state.player_armor -= 7,
                    ],
                ));
            },
        },
        Spell {
            id: Poison,
            cost: 173,
            func: |state| {
                state.effects.push((
                    Poison,
                    vec![
                        |state| state.boss_health -= 3,
                        |state| state.boss_health -= 3,
                        |state| state.boss_health -= 3,
                        |state| state.boss_health -= 3,
                        |state| state.boss_health -= 3,
                        |state| state.boss_health -= 3,
                    ],
                ));
            },
        },
        Spell {
            id: Recharge,
            cost: 229,
            func: |state| {
                state.effects.push((
                    Recharge,
                    vec![
                        |state| state.player_mana += 101,
                        |state| state.player_mana += 101,
                        |state| state.player_mana += 101,
                        |state| state.player_mana += 101,
                        |state| state.player_mana += 101,
                    ],
                ));
            },
        },
    ];
}

fn game_over(state: &Game) -> bool {
    state.boss_health <= 0 || state.player_health <= 0
}

fn apply_effects(state: &mut Game) {
    for i in 0..state.effects.len() {
        let f = state.effects[i].1.remove(0);
        f(state);
    }
    state.effects.retain(|elem| !elem.1.is_empty());
}

fn begin_turn(state: &mut Game) {
    if state.hard && state.player_turn {
        state.player_health -= 1;
    }
    apply_effects(state);
}

fn boss_attack(state: &mut Game) {
    state.player_health -= max(1, state.boss_damage - state.player_armor);
}

fn parse_boss(input: &str, hard: bool) -> Game {
    let v: Vec<i32> = input
        .lines()
        .map(|x| x.split(": ").last().unwrap().parse().unwrap())
        .collect();
    Game {
        player_health: 50,
        player_mana: 500,
        player_armor: 0,
        boss_health: v[0],
        boss_damage: v[1],
        player_turn: true,
        hard: hard,
        effects: Vec::new(),
    }
}

fn go(mana: i32, mut state: Game, mctw: &mut Option<i32>) {
    begin_turn(&mut state);
    if game_over(&state) {
        if state.boss_health <= 0 {
            mctw.replace(mana);
        }
    } else if !state.player_turn {
        boss_attack(&mut state);
        state.player_turn = !state.player_turn;
        go(mana, state, mctw);
    } else {
        for spell in SPELLS.iter() {
            if state.player_mana >= spell.cost
                && (mctw.is_none() || mana + spell.cost < mctw.unwrap())
                && !state.effects.iter().any(|x| x.0 == spell.id)
            {
                let mut state2 = state.clone();
                let f = spell.func;
                f(&mut state2);
                state2.player_mana -= spell.cost;
                state2.player_turn = !state2.player_turn;
                go(mana + spell.cost, state2, mctw);
            }
        }
    }
}

fn min_cost_to_win(state: Game) -> Option<i32> {
    let mut o = None;
    go(0, state, &mut o);
    o
}

pub fn part1(input: &str) -> Option<i32> {
    min_cost_to_win(parse_boss(input, false))
}

pub fn part2(input: &str) -> Option<i32> {
    min_cost_to_win(parse_boss(input, true))
}
