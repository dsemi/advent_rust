use crate::year2017::day16::Action::*;
use ahash::AHashMap;

enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_actions(input: &str) -> Vec<Action> {
    input
        .split(',')
        .map(|action| match action.chars().next().unwrap() {
            's' => Spin(action[1..].parse().unwrap()),
            'x' => {
                let (a, b) = action[1..].split_once('/').unwrap();
                Exchange(a.parse().unwrap(), b.parse().unwrap())
            }
            'p' => {
                let (a, b) = action[1..].split_once('/').unwrap();
                Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
            }
            _ => panic!("Invalid action: {}", action),
        })
        .collect()
}

fn apply_action(s: &mut String, action: &Action) {
    unsafe {
        match action {
            Spin(n) => {
                s.as_bytes_mut().rotate_right(*n);
            }
            Exchange(i, j) => {
                s.as_bytes_mut().swap(*i, *j);
            }
            Partner(a, b) => {
                let (i, j) = (s.find(*a).unwrap(), s.find(*b).unwrap());
                s.as_bytes_mut().swap(i, j);
            }
        }
    }
}

fn dance(n: usize, actions: Vec<Action>) -> String {
    let mut result = "abcdefghijklmnop".to_string();
    let mut tbl = AHashMap::new();
    for c in 0..n {
        if let Some(v) = tbl.insert(result.clone(), c) {
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
    result
}

pub fn part1(input: &str) -> String {
    dance(1, parse_actions(input))
}

pub fn part2(input: &str) -> String {
    dance(1_000_000_000, parse_actions(input))
}
