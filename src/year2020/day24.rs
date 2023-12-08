use crate::utils::parsers::*;
use crate::utils::C;
use ahash::{AHashMap, AHashSet};
use once_cell::sync::Lazy;
use std::cmp::{max, min};

static DIRS: Lazy<AHashMap<&str, C<i32>>> = Lazy::new(|| {
    let mut m = AHashMap::new();
    m.insert("e", C(1, -1));
    m.insert("se", C(0, -1));
    m.insert("sw", C(-1, 0));
    m.insert("w", C(-1, 1));
    m.insert("nw", C(0, 1));
    m.insert("ne", C(1, 0));
    m
});

fn flip_tiles(s: &str) -> AHashSet<C<i32>> {
    let mut tiles = AHashMap::new();
    for line in s.lines() {
        let tile = fold_repeat(
            0..,
            alt(("e", "w", "se", "sw", "nw", "ne")),
            || C(0, 0),
            |a, b| a + DIRS[b],
        )
        .read(line);
        *tiles.entry(tile).or_insert(0) += 1;
    }
    tiles
        .into_iter()
        .filter(|&(_, v)| v % 2 == 1)
        .map(|(t, _)| t)
        .collect()
}

pub fn part1(input: &str) -> usize {
    flip_tiles(input).len()
}

pub fn part2(input: &str) -> usize {
    const STEPS: i32 = 100;
    let tiles = flip_tiles(input);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = tiles.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(min_x, min_y, max_x, max_y), &C(x, y)| {
            (min(min_x, x), min(min_y, y), max(max_x, x), max(max_y, y))
        },
    );
    let x_offset = -min_x + STEPS + 1;
    let y_offset = -min_y + STEPS + 1;
    min_x += x_offset;
    min_y += y_offset;
    max_x += x_offset;
    max_y += y_offset;
    let mut grid = vec![vec![false; (max_x + STEPS + 2) as usize]; (max_y + STEPS + 2) as usize];
    for C(x, y) in &tiles {
        grid[(y + y_offset) as usize][(x + x_offset) as usize] = true;
    }
    let mut grid2 = grid.clone();
    for _ in 0..STEPS {
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        for r in min_y..=max_y {
            for c in min_x..=max_x {
                let adj = DIRS
                    .values()
                    .filter(|C(x, y)| grid[(r + y) as usize][(c + x) as usize])
                    .count();
                if grid[r as usize][c as usize] {
                    grid2[r as usize][c as usize] = adj != 0 && adj <= 2;
                } else {
                    grid2[r as usize][c as usize] = adj == 2;
                }
            }
        }
        std::mem::swap(&mut grid, &mut grid2);
    }
    grid.into_iter()
        .map(|row| row.into_iter().filter(|x| *x).count())
        .sum()
}
