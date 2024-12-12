use crate::utils::parsers::*;
use crate::utils::{Grid, C};
use hashbrown::{HashMap, HashSet};
use phf::phf_map;

const DIRS: phf::Map<&str, C<i32>> = phf_map! {
    "e"  => C(1, -1),
    "se" => C(0, -1),
    "sw" => C(-1, 0),
    "w"  => C(-1, 1),
    "nw" => C(0, 1),
    "ne" => C(1, 0),
};

fn flip_tiles(s: &str) -> HashSet<C<i32>> {
    let mut tiles = HashMap::new();
    for line in s.lines() {
        let tile = repeat(0.., alt(("e", "w", "se", "sw", "nw", "ne")))
            .fold(|| C(0, 0), |a, b| a + DIRS[b])
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
    let (mut min, mut max) = tiles.iter().fold(
        (C(i32::MAX, i32::MAX), C(i32::MIN, i32::MIN)),
        |(min, max), &pos| (min.smol(pos), max.swol(pos)),
    );
    let offset = -min + C(STEPS + 1, STEPS + 1);
    min += offset;
    max += offset;
    let mut grid = Grid::new(max.0 + STEPS + 2, max.1 + STEPS + 2);
    tiles.iter().for_each(|pos| grid[pos + offset] = true);
    let mut grid2 = grid.clone();
    for _ in 0..STEPS {
        min -= C(1, 1);
        max += C(1, 1);
        for r in min.0..=max.0 {
            for c in min.1..=max.1 {
                let pos = C(r, c);
                let adj = DIRS.values().filter(|&d| grid[pos + d]).count();
                if grid[pos] {
                    grid2[pos] = adj != 0 && adj <= 2;
                } else {
                    grid2[pos] = adj == 2;
                }
            }
        }
        std::mem::swap(&mut grid, &mut grid2);
    }
    grid.into_iter().filter(|&x| x).count()
}
