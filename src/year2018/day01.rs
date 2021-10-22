use ahash::AHashSet;

use crate::utils::*;

pub fn part1(input: &str) -> i64 {
    input.lines().map(|x| x.parse::<i64>().unwrap()).sum()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut s = AHashSet::new();
    input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .cycle()
        .good_scan(0, |st, x| *st + x)
        .find(|&x| !s.insert(x))
}
