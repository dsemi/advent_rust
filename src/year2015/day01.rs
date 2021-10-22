use crate::utils::*;

pub fn part1(input: &str) -> i64 {
    input.chars().map(|x| if x == '(' { 1 } else { -1 }).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .good_scan(0, |a, b| *a + b)
        .position(|x| x < 0)
}
