use crate::utils::*;
use ahash::AHashSet;

struct Garden {
    grid: Vec<Vec<u8>>,
    frontier: Vec<C<i32>>,
    visited: AHashSet<C<i32>>,
    len: i32,
    evens: usize,
    odds: usize,
    depth: usize,
}

fn garden(grid: Vec<Vec<u8>>) -> Garden {
    let len = grid.len() as i32;
    let start = C(len / 2, len / 2);
    Garden {
        grid,
        frontier: vec![start],
        visited: [start].into_iter().collect(),
        len,
        evens: 1,
        odds: 0,
        depth: 0,
    }
}

impl Iterator for Garden {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut tmp = Vec::new();
        for pos in &self.frontier {
            for d in [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)] {
                let p = pos + d;
                if self.grid[C(p.0.rem_euclid(self.len), p.1.rem_euclid(self.len))] != b'#'
                    && self.visited.insert(p)
                {
                    tmp.push(p);
                }
            }
        }
        self.frontier = tmp;
        self.depth += 1;
        if self.depth & 1 == 0 {
            self.evens += self.frontier.len();
            Some(self.odds)
        } else {
            self.odds += self.frontier.len();
            Some(self.evens)
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    garden(grid).nth(64).unwrap()
}

const GOAL: usize = 26501365;

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let len = grid.len();
    let mut spaces = garden(grid);
    let a0 = spaces.nth(len / 2).unwrap();
    let a1 = spaces.nth(len - 1).unwrap();
    let a2 = spaces.nth(len - 1).unwrap();
    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;
    let n = GOAL / len;
    b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
}
