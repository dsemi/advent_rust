use crate::utils::parsers::*;
use hashbrown::HashMap;
use itertools::{iterate, Itertools};
use rayon::prelude::*;
use std::iter::zip;

const MASK: i64 = (1 << 24) - 1;

fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) & MASK;
    secret ^= secret >> 5;
    (secret ^ secret << 11) & MASK
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(|p| iterate(p.i64(), step).nth(2000).unwrap()).sum()
}

fn idx(a: i64, b: i64, c: i64, d: i64) -> usize {
    (6859 * (a + 9) + 361 * (b + 9) + 19 * (c + 9) + d + 9) as usize
}

pub fn part2(input: &str) -> i64 {
    input
        .par_lines()
        .map(|p| {
            let mut dp = vec![0; 19usize.pow(4)];
            let ns: Vec<_> = iterate(p.i64(), step).take(2001).map(|n| n % 10).collect();
            ns.into_iter()
                .rev()
                .tuple_windows()
                .for_each(|(a, b, c, d, e)| dp[idx(d - e, c - d, b - c, a - b)] = a);
            dp
        })
        .reduce(|| vec![0; 19usize.pow(4)], |a, b| zip(a, b).map(|(a, b)| a + b).collect())
        .into_iter()
        .max()
        .unwrap()
}
