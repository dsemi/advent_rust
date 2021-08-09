use ahash::{AHashMap, AHashSet};
use regex::Regex;
use std::cmp::{max, min};

use crate::utils::Coord;

lazy_static! {
    static ref DIRS: AHashMap<&'static str, Coord<i32>> = {
        let mut m = AHashMap::new();
        m.insert("e", Coord::new(1, -1));
        m.insert("se", Coord::new(0, -1));
        m.insert("sw", Coord::new(-1, 0));
        m.insert("w", Coord::new(-1, 1));
        m.insert("nw", Coord::new(0, 1));
        m.insert("ne", Coord::new(1, 0));
        m
    };
}

fn flip_tiles(s: &str) -> AHashSet<Coord<i32>> {
    let re = Regex::new(
        &itertools::Itertools::intersperse(DIRS.keys().copied(), "|").collect::<String>(),
    )
    .unwrap();
    let mut tiles = AHashMap::new();
    for line in s.lines() {
        let tile = re.find_iter(line).map(|d| DIRS[d.as_str()]).sum();
        let e = tiles.entry(tile).or_insert(0);
        *e += 1;
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
        |(min_x, min_y, max_x, max_y), coord| {
            (
                min(min_x, coord.x),
                min(min_y, coord.y),
                max(max_x, coord.x),
                max(max_y, coord.y),
            )
        },
    );
    let x_offset = -min_x + STEPS + 1;
    let y_offset = -min_y + STEPS + 1;
    min_x += x_offset;
    min_y += y_offset;
    max_x += x_offset;
    max_y += y_offset;
    let mut grid = vec![vec![false; (max_x + STEPS + 2) as usize]; (max_y + STEPS + 2) as usize];
    for tile in &tiles {
        grid[(tile.y + y_offset) as usize][(tile.x + x_offset) as usize] = true;
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
                    .filter(|d| grid[(r as i32 + d.y) as usize][(c as i32 + d.x) as usize])
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
    grid.into_iter().map(|row| row.into_iter().filter(|x| *x).count()).sum()
}
