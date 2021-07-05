use std::cmp::min;
use std::collections::HashMap;

use crate::utils::Coord;

fn parse_edges(input: &str) -> HashMap<Coord<i32>, usize> {
    let mut stack = Vec::new();
    let mut pos = Coord::new(0, 0);
    let mut result = HashMap::new();
    for c in input[1..input.len()-1].chars() {
        match c {
            '(' => stack.push(pos),
            ')' => pos = stack.pop().unwrap(),
            '|' => pos = *stack.last().unwrap(),
            _ => {
                let dir = match c {
                    'N' => Coord::new(0, -1),
                    'E' => Coord::new(1, 0),
                    'S' => Coord::new(0, 1),
                    'W' => Coord::new(-1, 0),
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
    parse_edges(input).values().copied().max()
}

pub fn part2(input: &str) -> usize {
    parse_edges(input).values().filter(|&v| *v >= 1000).count()
}
