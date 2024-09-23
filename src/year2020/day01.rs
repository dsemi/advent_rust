use std::cmp::Ordering::*;

const TARGET: i64 = 2020;

fn two_sum(target: i64, entries: &[i64]) -> Option<i64> {
    let mut gen = entries.iter();
    let mut lo = gen.next();
    let mut hi = gen.next_back();
    while let (Some(a), Some(b)) = (lo, hi) {
        match (a + b).cmp(&target) {
            Equal => return Some(a * b),
            Less => lo = gen.next(),
            Greater => hi = gen.next_back(),
        }
    }
    None
}

pub fn part1(mut input: Vec<i64>) -> Option<i64> {
    input.sort_unstable();
    two_sum(TARGET, &input)
}

pub fn part2(mut input: Vec<i64>) -> Option<i64> {
    input.sort_unstable();
    (0..input.len()).find_map(|i| {
        let a = input[i];
        two_sum(TARGET - a, &input[i + 1..]).map(|p| a * p)
    })
}
