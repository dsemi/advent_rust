use crate::year2017::day22::NodeState::*;

#[derive(Clone, Eq, PartialEq)]
enum NodeState {
    Cleaned,
    Weakened,
    Infected,
    Flagged,
}

fn turn(d: u8, v: &NodeState) -> u8 {
    match v {
        Cleaned => (d + 3) % 4,
        Weakened => d,
        Infected => (d + 1) % 4,
        Flagged => (d + 2) % 4,
    }
}

fn count_infections(input: &str, bursts: usize, next: fn(&NodeState) -> NodeState) -> usize {
    const GRID_SIZE: usize = 1024;
    const MID: usize = GRID_SIZE / 2;
    let mut grid: Vec<NodeState> = vec![Cleaned; GRID_SIZE * GRID_SIZE];
    for (row, r) in input.lines().zip(MID - 12..=MID + 12) {
        for (v, c) in row.chars().zip(MID - 12..=MID + 12) {
            if v == '#' {
                grid[r * GRID_SIZE + c] = Infected;
            }
        }
    }
    let mut pos = MID * GRID_SIZE + MID;
    let mut dir = 3;
    let mut result = 0;
    for _ in 0..bursts {
        dir = turn(dir, &grid[pos]);
        grid[pos] = next(&grid[pos]);
        result += (grid[pos] == Infected) as usize;
        match dir {
            0 => pos += 1,
            1 => pos += GRID_SIZE,
            2 => pos -= 1,
            3 => pos -= GRID_SIZE,
            _ => panic!("Invalid dir: {}", dir),
        }
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
