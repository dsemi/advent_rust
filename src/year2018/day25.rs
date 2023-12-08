use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashSet;

type Pt = (i64, i64, i64, i64);

fn parse_points(input: &str) -> UnionFind<Pt> {
    input
        .lines()
        .map(|line| sep_tuple4(i64, ',').read(line))
        .collect()
}

fn dist((w0, x0, y0, z0): Pt, (w1, x1, y1, z1): Pt) -> i64 {
    (w0 - w1).abs() + (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn constellations(mut pts: UnionFind<Pt>) -> usize {
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            if dist(pts[i], pts[j]) <= 3 {
                pts.union(i, j);
            }
        }
    }
    (0..pts.len())
        .map(|p| pts.find(p))
        .collect::<AHashSet<_>>()
        .len()
}

pub fn part1(input: &str) -> usize {
    constellations(parse_points(input))
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
