use ahash::AHashMap;
use scan_fmt::scan_fmt as scanf;
use std::cmp::{max, min};

fn solve(input: &str, lo: i64, hi: i64) -> i64 {
    let mut cubes = AHashMap::new();
    for line in input.lines() {
        let (w, nx0, nx1, ny0, ny1, nz0, nz1) = scanf!(
            line,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let mut update = AHashMap::new();
        for ((ex0, ex1, ey0, ey1, ez0, ez1), es) in cubes.iter() {
            let (x0, x1) = (max(nx0, *ex0), min(nx1, *ex1));
            let (y0, y1) = (max(ny0, *ey0), min(ny1, *ey1));
            let (z0, z1) = (max(nz0, *ez0), min(nz1, *ez1));
            if x0 <= x1 && y0 <= y1 && z0 <= z1 {
                *update.entry((x0, x1, y0, y1, z0, z1)).or_insert(0) -= *es;
            }
        }
        if w == "on" {
            *update.entry((nx0, nx1, ny0, ny1, nz0, nz1)).or_insert(0) += 1;
        }
        for (k, v) in update {
            *cubes.entry(k).or_insert(0) += v;
        }
    }
    cubes
        .into_iter()
        .map(|((x0, x1, y0, y1, z0, z1), s)| {
            let (x0, x1) = (max(lo, x0), min(hi, x1));
            let (y0, y1) = (max(lo, y0), min(hi, y1));
            let (z0, z1) = (max(lo, z0), min(hi, z1));
            max(0, x1 - x0 + 1) * max(0, y1 - y0 + 1) * max(0, z1 - z0 + 1) * s
        })
        .sum()
}

pub fn part1(input: &str) -> i64 {
    solve(input, -50, 50)
}

pub fn part2(input: &str) -> i64 {
    solve(input, i64::MIN, i64::MAX)
}
