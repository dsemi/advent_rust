use crate::utils::C;
use crate::utils::parsers::*;
use hashbrown::{HashMap, HashSet};
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
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
    tiles.into_iter().filter(|&(_, v)| v % 2 == 1).map(|(t, _)| t).collect()
}

pub fn part1(input: &str) -> usize {
    flip_tiles(input).len()
}

pub fn part2(input: &str) -> usize {
    let tiles = flip_tiles(input);
    let (C(r0, c0), C(r1, c1)) =
        tiles.iter().fold((C(i32::MAX, i32::MAX), C(i32::MIN, i32::MIN)), |(min, max), &pos| {
            (min.smol(pos), max.swol(pos))
        });
    let kernel: Array2<u8> = array![[0, 1, 1], [1, 7, 1], [1, 1, 0]];
    let mut grid: Array2<u8> = Array2::zeros(((r1 - r0 + 1) as usize, (c1 - c0 + 1) as usize));
    tiles.iter().map(|C(r, c)| ((r - r0) as usize, (c - c0) as usize)).for_each(|i| grid[i] = 1);
    for _ in 0..100 {
        grid = grid.conv(&kernel, ConvMode::Full, PaddingMode::Zeros).unwrap();
        grid.mapv_inplace(|v| u8::from(v == 2 || v == 8 || v == 9));
    }
    grid.into_iter().filter(|&v| v > 0).count()
}
