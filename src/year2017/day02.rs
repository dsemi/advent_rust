use crate::utils::parsers2::*;
use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(i64::read)
                .minmax()
                .into_option()
                .map(|(mn, mx)| mx - mn)
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(i64::read)
                .combinations(2)
                .find_map(|x| {
                    (x[0] % x[1] == 0)
                        .then(|| x[0] / x[1])
                        .or_else(|| (x[1] % x[0] == 0).then(|| x[1] / x[0]))
                })
                .unwrap()
        })
        .sum()
}
