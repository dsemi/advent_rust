use ahash::AHashMap;
use ndarray::{Array, Axis, Dim};
use std::collections::hash_map::Entry::*;

type Grid = Array<u8, Dim<[usize; 2]>>;

fn tilt(grid: &mut Grid, axis: Axis, d: i32) {
    grid.axis_iter_mut(axis).for_each(|mut sl| {
        let len = sl.len();
        let mut to = if d > 0 { len - 1 } else { 0 };
        for i in (0..len).map(|i| if d > 0 { len - 1 - i } else { i }) {
            if sl[i] == b'O' {
                sl[i] = b'.';
                sl[to] = b'O';
                to -= d as usize;
            } else if sl[i] == b'#' {
                to = i - d as usize;
            }
        }
    });
}

fn load(grid: &Grid) -> usize {
    let rows = grid.len_of(Axis(0));
    grid.axis_iter(Axis(0))
        .enumerate()
        .map(|(i, row)| (rows - i) * row.iter().filter(|&v| *v == b'O').count())
        .sum()
}

fn parse(input: &str) -> Grid {
    let mut res = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    for line in input.lines() {
        cols = line.len();
        rows += 1;
        res.extend(line.bytes());
    }
    Array::from_shape_vec((rows, cols), res).unwrap()
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    tilt(&mut grid, Axis(1), -1);
    load(&grid)
}

const CYCLES: usize = 1000000000;

fn cycle(grid: &mut Grid) {
    tilt(grid, Axis(1), -1);
    tilt(grid, Axis(0), -1);
    tilt(grid, Axis(1), 1);
    tilt(grid, Axis(0), 1);
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut visited = AHashMap::new();
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
    load(&grid)
}
