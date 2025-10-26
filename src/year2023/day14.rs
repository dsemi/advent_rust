use hashbrown::{HashMap, hash_map::Entry::*};
use std::ops::{Shl, Shr};

enum Dir {
    North,
    West,
    South,
    East,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Grid {
    rows: usize,
    cols: usize,
    cube: Vec<u128>,
    round: Vec<u128>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut cube = Vec::new();
        let mut round = Vec::new();
        let mut cols = 0;
        for line in input.lines() {
            cols = line.len();
            let mut cs = 0;
            let mut rs = 0;
            for b in line.bytes() {
                cs <<= 1;
                rs <<= 1;
                match b {
                    b'#' => cs |= 1,
                    b'O' => rs |= 1,
                    _ => (),
                }
            }
            cube.push(cs);
            round.push(rs);
        }
        assert!(cols <= 128);
        Self { rows: cube.len(), cols, cube, round }
    }

    fn tilt_ns(&mut self, d: i32) {
        let d = d as usize;
        let mut moved = true;
        while std::mem::take(&mut moved) {
            for r in 1..self.rows {
                let r = if d == 1 { self.rows - 1 - r } else { r };
                let can_move = self.round[r] & !self.round[r + d] & !self.cube[r + d];
                self.round[r] &= !can_move;
                self.round[r + d] |= can_move;
                moved |= can_move != 0;
            }
        }
    }

    fn tilt_ew(&mut self, col: u128, f: fn(u128, u8) -> u128, s: fn(u128, u8) -> u128) {
        let mut moved = true;
        while std::mem::take(&mut moved) {
            for (r, c) in self.round.iter_mut().zip(self.cube.iter()) {
                let can_move = *r & !(f(*r | c, 1)) & !col;
                *r = *r & !can_move | s(can_move, 1);
                moved |= can_move != 0;
            }
        }
    }

    fn tilt(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.tilt_ns(-1),
            Dir::West => self.tilt_ew(1 << (self.cols - 1), u128::shr, u128::shl),
            Dir::South => self.tilt_ns(1),
            Dir::East => self.tilt_ew(1, u128::shl, u128::shr),
        }
    }

    fn load(&self) -> usize {
        self.round
            .iter()
            .enumerate()
            .map(|(r, row)| (self.rows - r) * row.count_ones() as usize)
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.tilt(Dir::North);
    grid.load()
}

const CYCLES: usize = 1_000_000_000;

fn cycle(grid: &mut Grid) {
    grid.tilt(Dir::North);
    grid.tilt(Dir::West);
    grid.tilt(Dir::South);
    grid.tilt(Dir::East);
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    let mut visited = HashMap::new();
    for i in 1..=CYCLES {
        cycle(&mut grid);
        match visited.entry(grid.clone()) {
            Occupied(e) => {
                let cycle_len = i - e.get();
                let remaining = (CYCLES - i) % cycle_len;
                for _ in 0..remaining {
                    cycle(&mut grid);
                }
                break;
            }
            Vacant(e) => {
                e.insert(i);
            }
        }
    }
    grid.load()
}
