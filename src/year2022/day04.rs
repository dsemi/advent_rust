use crate::utils::parsers::*;

fn solve(input: &str, f: fn(i32, i32, i32, i32) -> bool) -> usize {
    lines_iter(input, coord(sep2(i32, '-')))
        .filter(|&((a0, a1), (b0, b1))| f(a0, a1, b0, b1))
        .count()
}

pub fn part1(input: &str) -> usize {
    solve(input, |a0, a1, b0, b1| {
        a0 <= b0 && a1 >= b1 || b0 <= a0 && b1 >= a1
    })
}

pub fn part2(input: &str) -> usize {
    solve(input, |a0, a1, b0, b1| a0 <= b1 && b0 <= a1)
}
