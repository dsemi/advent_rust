use crate::utils::*;
use ahash::{AHashMap, AHashSet};

fn parse_pipes(input: &str) -> AHashMap<i64, Vec<i64>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" <-> ").unwrap();
            (
                a.parse().unwrap(),
                b.split(", ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let m = parse_pipes(input);
    bfs(0, |n| m[n].clone()).count()
}

pub fn part2(input: &str) -> usize {
    let m = parse_pipes(input);
    let mut seen = AHashSet::new();
    m.keys()
        .filter_map(|n| {
            (!seen.contains(n)).then(|| seen.extend(bfs(*n, |x| m[x].clone()).map(|x| x.1)))
        })
        .count()
}
