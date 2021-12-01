use ahash::AHashSet;

use crate::utils::*;

pub fn part1(ns: Vec<i64>) -> i64 {
    ns.into_iter().sum()
}

pub fn part2(ns: Vec<i64>) -> Option<i64> {
    let mut s = AHashSet::new();
    ns.into_iter()
        .cycle()
        .good_scan(0, |st, x| *st + x)
        .find(|&x| !s.insert(x))
}
