use crate::utils::parsers::*;
use std::cmp::min;

pub fn part1(input: &str) -> i32 {
    let mut ns: Vec<i32> = input.split(',').map(int).collect();
    ns.sort_unstable();
    let med = if ns.len() % 2 == 0 {
        (ns[ns.len() / 2 - 1] + ns[ns.len() / 2]) / 2
    } else {
        ns[ns.len() / 2]
    };
    ns.into_iter().map(|n| (n - med).abs()).sum()
}

fn g(n: i32) -> i32 {
    n * (n + 1) / 2
}

pub fn part2(input: &str) -> i32 {
    let ns: Vec<i32> = input.split(',').map(int).collect();
    let mean = (ns.iter().sum::<i32>() as f32) / ns.len() as f32;
    min(
        ns.iter().map(|n| g((n - mean.floor() as i32).abs())).sum(),
        ns.iter().map(|n| g((n - mean.ceil() as i32).abs())).sum(),
    )
}
