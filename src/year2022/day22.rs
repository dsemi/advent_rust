use crate::utils::*;
use itertools::iterate;
use regex::Regex;

enum Instr {
    Turn(bool),
    Step(usize),
}

struct Board {
    grid: Vec<Vec<char>>,
    path: Vec<Instr>,
    top_left: C<i32>,
}

impl Board {
    fn new(input: &str) -> Self {
        let pts = input.split("\n\n").collect::<Vec<_>>();
        let reg = Regex::new(r"\d+|.").unwrap();
        let grid: Vec<Vec<char>> = pts[0].lines().map(|line| line.chars().collect()).collect();
        Self {
            top_left: C(0, grid[0].iter().position(|&c| c != ' ').unwrap() as i32),
            grid,
            path: reg
                .find_iter(pts[1])
                .map(|instr| match instr.as_str() {
                    "L" => Instr::Turn(false),
                    "R" => Instr::Turn(true),
                    n => Instr::Step(n.parse().unwrap()),
                })
                .collect(),
        }
    }

    fn walk<F>(&self, step: F) -> i32
    where
        F: Fn(&[Vec<char>], C<i32>, C<i32>) -> (C<i32>, C<i32>),
    {
        let mut pos = iterate(self.top_left, |p| p + C(0, 1))
            .find(|&p| self.grid[p] == '.')
            .unwrap();
        let mut dir = C(0, 1);
        for instr in self.path.iter() {
            match instr {
                Instr::Turn(false) => dir *= C(0, 1),
                Instr::Turn(true) => dir *= C(0, -1),
                Instr::Step(n) => {
                    for _ in 0..*n {
                        let (pos2, dir2) = step(&self.grid, pos, dir);
                        if self.grid[pos2] == '#' {
                            break;
                        }
                        (pos, dir) = (pos2, dir2);
                    }
                }
            }
        }
        let C(row, col) = pos + C(1, 1);
        let facing = match dir {
            C(0, 1) => 0,
            C(1, 0) => 1,
            C(0, -1) => 2,
            C(-1, 0) => 3,
            _ => unreachable!(),
        };
        1000 * row + 4 * col + facing
    }
}

pub fn part1(input: &str) -> i32 {
    Board::new(input).walk(|grid, C(r, c), C(dr, dc)| {
        let pos = if dr != 0 {
            let mr = grid.len() as i32;
            iterate(r, |r| (r + dr).rem_euclid(mr))
                .map(|r| C(r, c))
                .skip(1)
                .find(|&p| p.index_of(grid).filter(|&p| grid[p] != ' ').is_some())
                .unwrap()
        } else {
            let mc = grid[r as usize].len() as i32;
            iterate(c, |c| (c + dc).rem_euclid(mc))
                .map(|c| C(r, c))
                .skip(1)
                .find(|&p| p.index_of(grid).filter(|&p| grid[p] != ' ').is_some())
                .unwrap()
        };
        (pos, C(dr, dc))
    })
}

pub fn part2(input: &str) -> i32 {
    let board = Board::new(input);
    board.walk(|_, pos, dir| match (pos, dir) {
        (C(0, c), C(-1, 0)) if (50..100).contains(&c) => (C(c + 100, 0), C(0, 1)),
        (C(r, 0), C(0, -1)) if (150..200).contains(&r) => (C(0, r - 100), C(1, 0)),
        (C(0, c), C(-1, 0)) if (100..150).contains(&c) => (C(199, c - 100), C(-1, 0)),
        (C(199, c), C(1, 0)) if (0..50).contains(&c) => (C(0, c + 100), C(1, 0)),
        (C(r, 50), C(0, -1)) if (0..50).contains(&r) => (C(149 - r, 0), C(0, 1)),
        (C(r, 0), C(0, -1)) if (100..150).contains(&r) => (C(149 - r, 50), C(0, 1)),
        (C(r, 149), C(0, 1)) if (0..50).contains(&r) => (C(149 - r, 99), C(0, -1)),
        (C(r, 99), C(0, 1)) if (100..150).contains(&r) => (C(149 - r, 149), C(0, -1)),
        (C(49, c), C(1, 0)) if (100..150).contains(&c) => (C(c - 50, 99), C(0, -1)),
        (C(r, 99), C(0, 1)) if (50..100).contains(&r) => (C(49, r + 50), C(-1, 0)),
        (C(r, 50), C(0, -1)) if (50..100).contains(&r) => (C(100, r - 50), C(1, 0)),
        (C(100, c), C(-1, 0)) if (0..50).contains(&c) => (C(c + 50, 50), C(0, 1)),
        (C(149, c), C(1, 0)) if (50..100).contains(&c) => (C(c + 100, 49), C(0, -1)),
        (C(r, 49), C(0, 1)) if (150..200).contains(&r) => (C(149, r - 100), C(-1, 0)),
        _ => (pos + dir, dir),
    })
}
