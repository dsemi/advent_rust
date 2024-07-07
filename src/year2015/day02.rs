use crate::utils::parsers::*;

fn process(input: &str, f: fn((i32, i32, i32)) -> i32) -> i32 {
    input.lines().map(|line| f(sep3(i32, 'x').read(line))).sum()
}

pub fn part1(input: &str) -> i32 {
    process(input, |(l, w, h)| {
        2 * (l * w + l * h + w * h) + (l * w).min(l * h).min(w * h)
    })
}

pub fn part2(input: &str) -> i32 {
    process(input, |(l, w, h)| {
        l * w * h + 2 * (l + w).min(l + h).min(w + h)
    })
}
