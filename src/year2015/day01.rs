use crate::utils::*;

pub fn part1(input: &str) -> i64 {
    input.chars().map(|x| if x == '(' { 1 } else { -1 }).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scanl(0, |acc, x| { acc + x })
        .position(|x| x < 0)
}
