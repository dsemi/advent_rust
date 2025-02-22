use crate::utils::*;
use std::iter::once;

const U: u8 = 0b1000;
const L: u8 = 0b0100;
const D: u8 = 0b0010;
const R: u8 = 0b0001;

fn turn(dir: u8, pipe: char) -> u8 {
    let dir = ((dir & 3) << 2) | ((dir & 12) >> 2);
    let pipe = match pipe {
        '|' => U | D,
        '-' => L | R,
        'L' => U | R,
        'J' => U | L,
        '7' => D | L,
        'F' => D | R,
        _ => unreachable!(),
    };
    dir ^ pipe
}

fn coord(dir: u8) -> C<i32> {
    match dir {
        U => C(-1, 0),
        L => C(0, -1),
        D => C(1, 0),
        R => C(0, 1),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (C<i32>, u8, Grid<char, i32>) {
    let grid: Grid<_, i32> = input.chars().collect();
    let start = grid.idx_iter().find_map(|(C(r, c), &v)| (v == 'S').then_some(C(r, c))).unwrap();
    let dir = [U, L, D, R]
        .into_iter()
        .find(|&d| {
            grid.get(start + coord(d)).filter(|&&v| (turn(d, v)).count_ones() == 1).is_some()
        })
        .unwrap();
    (start, dir, grid)
}

fn main_pts(input: &str) -> impl Iterator<Item = C<i32>> {
    let (pos, dir, grid) = parse(input);
    once(pos).chain((0..).scan((pos, dir), move |acc, _| {
        acc.0 += coord(acc.1);
        if grid[acc.0] == 'S' {
            return None;
        }
        acc.1 = turn(acc.1, grid[acc.0]);
        Some(acc.0)
    }))
}

pub fn part1(input: &str) -> usize {
    main_pts(input).count() / 2
}

pub fn part2(input: &str) -> usize {
    let pts: Vec<_> = main_pts(input).collect();
    let area = shoelace(&pts);
    picks_interior(area as usize, pts.len())
}
