use crate::utils::parsers::*;
use hashbrown::HashMap;
use itertools::{iterate, Itertools};
use rayon::prelude::*;

fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) % 16777216;
    secret = (secret ^ secret >> 5) % 16777216;
    (secret ^ secret << 11) % 16777216
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|p| iterate(p.i64(), step).nth(2000).unwrap()).sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .par_lines()
        .map(|p| {
            let ns: Vec<_> = iterate(p.i64(), step).take(2001).map(|n| n % 10).collect();
            ns.into_iter()
                .rev()
                .tuple_windows()
                .map(|(a, b, c, d, e)| ((d - e, c - d, b - c, a - b), a))
                .collect::<HashMap<_, _>>()
        })
        .reduce(
            || HashMap::new(),
            |mut acc, m| {
                m.into_iter().for_each(|(k, v)| *acc.entry(k).or_insert(0) += v);
                acc
            },
        )
        .into_values()
        .max()
        .unwrap()
}
