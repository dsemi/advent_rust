use std::collections::VecDeque;

use crate::year2017::day25::Dir::L;
use crate::year2017::day25::Dir::R;
use crate::year2017::day25::State::A;
use crate::year2017::day25::State::B;
use crate::year2017::day25::State::C;
use crate::year2017::day25::State::D;
use crate::year2017::day25::State::E;
use crate::year2017::day25::State::F;

#[derive(Clone, Copy)]
enum State {
    A, B, C, D, E, F
}

#[derive(Clone, Copy)]
enum Dir {
    L, R
}

fn go(d: Dir, tape: &mut VecDeque<i64>, i: &mut usize) {
    match d {
        L => if *i == 0 {
            tape.push_front(0);
        } else {
            *i -= 1;
        },
        R => {
            *i += 1;
            if *i >= tape.len() {
                tape.push_back(0);
            }
        },
    }
}

struct Rule {
    write: i64,
    dir: Dir,
    state: State,
}

fn make_rules() -> [[Rule; 2]; 6] {
    [
        [Rule { write: 1, dir: R, state: B }, Rule { write: 0, dir: L, state: F }],
        [Rule { write: 0, dir: R, state: C }, Rule { write: 0, dir: R, state: D }],
        [Rule { write: 1, dir: L, state: D }, Rule { write: 1, dir: R, state: E }],
        [Rule { write: 0, dir: L, state: E }, Rule { write: 0, dir: L, state: D }],
        [Rule { write: 0, dir: R, state: A }, Rule { write: 1, dir: R, state: C }],
        [Rule { write: 1, dir: L, state: A }, Rule { write: 1, dir: R, state: A }],
    ]
}

fn process(steps: usize) -> i64 {
    let mut tape = VecDeque::new();
    tape.push_back(0);
    let mut i = 0;
    let mut state = A;
    let rules = make_rules();
    for _ in 0..steps {
        let rule = &rules[state as usize][tape[i] as usize];
        tape[i] = rule.write;
        go(rule.dir, &mut tape, &mut i);
        state = rule.state;
    }
    tape.into_iter().sum()
}

pub fn part1(_input: &str) -> i64 {
    process(12794428)
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}
