use crate::utils::C;
use hashbrown::HashMap;
use std::cmp::min;

fn parse_edges(input: &str) -> HashMap<C<i32>, usize> {
    let mut stack = Vec::new();
    let mut pos = C(0, 0);
    let mut result = HashMap::new();
    for c in input[1..input.len() - 1].chars() {
        match c {
            '(' => stack.push(pos),
            ')' => pos = stack.pop().unwrap(),
            '|' => pos = *stack.last().unwrap(),
            _ => {
                let dir = match c {
                    'N' => C(0, -1),
                    'E' => C(1, 0),
                    'S' => C(0, 1),
                    'W' => C(-1, 0),
                    _ => panic!("Invalid dir: {}", c),
                };
                let v = result.get(&pos).unwrap_or(&0) + 1;
                pos += dir;
                let e = result.entry(pos).or_insert(v);
                *e = min(*e, v);
            }
        }
    }
    result
}

pub fn part1(input: &str) -> Option<usize> {
    parse_edges(input).into_values().max()
}

pub fn part2(input: &str) -> usize {
    parse_edges(input).values().filter(|&v| *v >= 1000).count()
}
