use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::utils::Coord;

lazy_static! {
    static ref DIRS: HashMap<&'static str, Coord<i64>> = {
        let mut m = HashMap::new();
        m.insert("e", Coord::new(1, -1));
        m.insert("se", Coord::new(0, -1));
        m.insert("sw", Coord::new(-1, 0));
        m.insert("w", Coord::new(-1, 1));
        m.insert("nw", Coord::new(0, 1));
        m.insert("ne", Coord::new(1, 0));
        m
    };
}

fn flip_tiles(s: &str) -> HashSet<Coord<i64>> {
    let re = Regex::new(
        &itertools::Itertools::intersperse(DIRS.keys().copied(), "|").collect::<String>(),
    )
    .unwrap();
    let mut tiles = HashMap::new();
    for line in s.lines() {
        let tile = re
            .find_iter(line)
            .map(|d| DIRS[d.as_str()])
            .sum();
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
    let mut tiles = flip_tiles(input);
    for _ in 0..100 {
        let mut adj = HashMap::new();
        for t in &tiles {
            for d in DIRS.values() {
                let e = adj.entry(t + d).or_insert(0);
                *e += 1;
            }
        }
        tiles = adj
            .into_iter()
            .filter(|&(t, v)| {
                if tiles.contains(&t) {
                    v != 0 && v <= 2
                } else {
                    v == 2
                }
            })
            .map(|x| x.0)
            .collect();
    }
    tiles.len()
}
