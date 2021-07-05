use lazy_static::lazy_static;

use crate::utils::Coord;
use crate::year2017::day22::NodeState::*;

#[derive(Clone, Eq, PartialEq)]
enum NodeState {
    Cleaned,
    Weakened,
    Infected,
    Flagged,
}

fn turn(d: &mut Coord<i32>, v: &NodeState) {
    lazy_static! {
        static ref LEFT: Coord<i32> = Coord::new(0, 1);
        static ref RIGHT: Coord<i32> = Coord::new(0, -1);
    }
    match v {
        Cleaned => *d *= *LEFT,
        Weakened => (),
        Infected => *d *= *RIGHT,
        Flagged => *d *= *RIGHT * *RIGHT,
    }
}

fn count_infections(input: &str, bursts: usize, next: fn(&NodeState) -> NodeState) -> usize {
    let mut grid: Vec<Vec<NodeState>> = vec![vec![Cleaned; 10001]; 10001];
    for (row, r) in input.lines().zip(4988..5013) {
        for (v, c) in row.chars().zip(4988..5013) {
            if v == '#' {
                grid[r][c] = Infected;
            }
        }
    }
    let mut pos = Coord::new(5000, 5000);
    let mut dir = Coord::new(-1, 0);
    let mut result = 0;
    for _ in 0..bursts {
        let val = &grid[pos.x as usize][pos.y as usize];
        turn(&mut dir, val);
        let val2 = next(val);
        result += (val2 == Infected) as usize;
        grid[pos.x as usize][pos.y as usize] = val2;
        pos += dir;
    }
    result
}

pub fn part1(input: &str) -> usize {
    count_infections(input, 10_000, |v| match v {
        Cleaned => Infected,
        Infected => Cleaned,
        _ => panic!("Invalid state"),
    })
}

pub fn part2(input: &str) -> usize {
    count_infections(input, 10_000_000, |v| match v {
        Cleaned => Weakened,
        Weakened => Infected,
        Infected => Flagged,
        Flagged => Cleaned,
    })
}
