use crate::utils::parsers::*;
use Action::*;
use hashbrown::HashMap;
use std::fmt::{Display, Error, Formatter, Write};

enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

struct Line {
    offset: usize,
    s: String,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..self.s.len() {
            f.write_char(self.s.as_bytes()[(self.offset + i).rem_euclid(self.s.len())] as char)?;
        }
        Ok(())
    }
}

fn parse_actions(input: &str) -> Vec<Action> {
    input
        .split(',')
        .map(|action| match action.chars().next().unwrap() {
            's' => Spin(action[1..].usize()),
            'x' => {
                let (a, b) = action[1..].split_once('/').unwrap();
                Exchange(a.usize(), b.usize())
            }
            'p' => {
                let (a, b) = action[1..].split_once('/').unwrap();
                Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
            }
            _ => panic!("Invalid action: {}", action),
        })
        .collect()
}

fn apply_action(l: &mut Line, action: &Action) {
    unsafe {
        match action {
            Spin(n) => {
                l.offset -= *n;
            }
            Exchange(i, j) => {
                let len = l.s.len();
                l.s.as_bytes_mut()
                    .swap((l.offset + *i).rem_euclid(len), (l.offset + *j).rem_euclid(len));
            }
            Partner(a, b) => {
                let (i, j) = (l.s.find(*a).unwrap(), l.s.find(*b).unwrap());
                l.s.as_bytes_mut().swap(i, j);
            }
        }
    }
}

fn dance(n: usize, actions: Vec<Action>) -> String {
    let mut result = Line { offset: 0, s: "abcdefghijklmnop".to_string() };
    let mut tbl = HashMap::new();
    for c in 0..n {
        if let Some(v) = tbl.insert(result.to_string(), c) {
            for _ in 0..((n - c) % (c - v)) {
                for action in &actions {
                    apply_action(&mut result, action);
                }
            }
            break;
        }
        for action in &actions {
            apply_action(&mut result, action);
        }
    }
    result.to_string()
}

pub fn part1(input: &str) -> String {
    dance(1, parse_actions(input))
}

pub fn part2(input: &str) -> String {
    dance(1_000_000_000, parse_actions(input))
}
