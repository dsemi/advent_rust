use crate::utils::*;
use std::iter::once;

const UP: u8 = 0b1000;
const DOWN: u8 = 0b0100;
const LEFT: u8 = 0b0010;
const RIGHT: u8 = 0b0001;

fn coord(dir: u8) -> C<i32> {
    match dir {
        UP => C(-1, 0),
        DOWN => C(1, 0),
        LEFT => C(0, -1),
        RIGHT => C(0, 1),
        _ => unreachable!(),
    }
}

fn invert(dir: u8) -> u8 {
    match dir {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        _ => unreachable!(),
    }
}

fn dirs(v: char) -> u8 {
    match v {
        '|' => UP | DOWN,
        '-' => LEFT | RIGHT,
        'L' => UP | RIGHT,
        'J' => UP | LEFT,
        '7' => DOWN | LEFT,
        'F' => DOWN | RIGHT,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (C<i32>, u8, Grid<char, i32>) {
    let grid: Grid<_, i32> = input.chars().collect();
    let start = grid
        .idx_iter()
        .find_map(|(C(r, c), &v)| (v == 'S').then_some(C(r, c)))
        .unwrap();
    let dir = if matches!(grid.get(start + coord(UP)), Some('|' | '7' | 'F')) {
        UP
    } else if matches!(grid.get(start + coord(DOWN)), Some('|' | 'L' | 'J')) {
        DOWN
    } else if matches!(grid.get(start + coord(LEFT)), Some('-' | 'L' | 'F')) {
        LEFT
    } else {
        RIGHT
    };
    (start, dir, grid)
}

fn main_pts(input: &str) -> impl Iterator<Item = C<i32>> {
    let (pos, dir, grid) = parse(input);
    once(pos).chain((0..).scan((pos, dir), move |acc, _| {
        acc.0 += coord(acc.1);
        if grid[acc.0] == 'S' {
            return None;
        }
        acc.1 = invert(acc.1) ^ dirs(grid[acc.0]);
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
