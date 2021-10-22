use ahash::AHashMap;
use take_until::TakeUntilExt;

use crate::utils::*;

fn mid_pt(x: i64, y: i64) -> i64 {
    (x - y) / 2 + y
}

fn corners() -> impl Iterator<Item = i64> {
    (1..).flat_map(|i| vec![i; 2]).good_scan(1, |a, b| *a + b)
}

pub fn part1(input: &str) -> i64 {
    let n = input.parse().unwrap();
    let ns: Vec<i64> = corners().take_until(|&c| c >= n).collect();
    let a = ns[ns.len() - 1];
    let b = ns[ns.len() - 2];
    let c = ns[ns.len() - 3];
    b - mid_pt(b, c) + (n - mid_pt(a, b)).abs()
}

fn spiral_path() -> impl Iterator<Item = i64> {
    let mut tbl = AHashMap::new();
    tbl.insert(Coord::new(0, 0), 1);
    (1..)
        .flat_map(|i| vec![i; 2])
        .zip(
            vec![
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(-1, 0),
                Coord::new(0, -1),
            ]
            .into_iter()
            .cycle(),
        )
        .flat_map(|(n, d)| vec![d; n])
        .scan((tbl, Coord::new(0, 0)), |(m, pos), dir| {
            *pos += dir;
            let val = adjacents(*pos).map(|c| *m.get(&c).unwrap_or(&0)).sum();
            m.insert(*pos, val);
            Some(val)
        })
}

pub fn part2(input: &str) -> Option<i64> {
    let n = input.parse().unwrap();
    spiral_path().find(|&x| x > n)
}
