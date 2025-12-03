use crate::utils::*;

fn solve(input: &str, ns: usize) -> u64 {
    Grid::<u64>::ints(input.bytes()).rows().fold(0, |sum, row| {
        let mut start = 0;
        (row.len() - ns + 1..=row.len()).fold(0, |curr, end| {
            start = (start..end).rev().max_by_key(|&i| row[i]).unwrap() + 1;
            10 * curr + row[start - 1]
        }) + sum
    })
}

pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 12)
}
