use crate::utils::parsers::*;
use rayon::prelude::*;

fn solve(input: &str, invalid: fn(Vec<u8>) -> bool) -> u64 {
    list(sep2(u64, '-'))
        .read(input)
        .into_par_iter()
        .map(|(lo, hi)| {
            (lo..=hi).into_par_iter().filter(|n| invalid(n.to_string().into_bytes())).sum::<u64>()
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    solve(input, |s| s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..])
}

pub fn part2(input: &str) -> u64 {
    solve(input, |s| {
        (1..=s.len() / 2).filter(|i| s.len() % i == 0).any(|i| {
            let (a, rest) = s.split_at(i);
            rest.chunks_exact(i).all(|c| a == c)
        })
    })
}
