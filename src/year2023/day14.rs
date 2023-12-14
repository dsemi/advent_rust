use crate::utils::*;
use ahash::AHashMap;
use std::collections::hash_map::Entry::*;
use std::ops::{Index, IndexMut};

fn tilt<G>(mut grid: G, outer: usize, inner: usize, d: i32)
where
    G: Index<(usize, usize), Output = u8> + IndexMut<(usize, usize)>,
{
    for o in 0..outer {
        let mut gen = (0..inner)
            .map(|i| if d > 0 { inner - 1 - i } else { i })
            .peekable();
        let mut to = *gen.peek().unwrap() as i32;
        for i in gen {
            if grid[(o, i)] == b'O' {
                if i as i32 != to {
                    grid[(o, i)] = b'.';
                    grid[(o, to as usize)] = b'O'
                }
                to -= d;
            } else if grid[(o, i)] == b'#' {
                to = i as i32 - d;
            }
        }
    }
}

fn load(grid: &[Vec<u8>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(r, row)| (grid.len() - r) * row.iter().filter(|&v| *v == b'O').count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    tilt(ColMajor(&mut grid), cols, rows, -1);
    load(&grid)
}

const CYCLES: usize = 1000000000;

fn cycle(grid: &mut Vec<Vec<u8>>, rows: usize, cols: usize) {
    tilt(ColMajor(grid), cols, rows, -1);
    tilt(RowMajor(grid), rows, cols, -1);
    tilt(ColMajor(grid), cols, rows, 1);
    tilt(RowMajor(grid), rows, cols, 1);
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = AHashMap::new();
    for i in 1..=CYCLES {
        cycle(&mut grid, rows, cols);
        match visited.entry(grid.clone()) {
            Occupied(e) => {
                let cycle_len = i - e.get();
                let remaining = (CYCLES - i) % cycle_len;
                for _ in 0..remaining {
                    cycle(&mut grid, rows, cols);
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
