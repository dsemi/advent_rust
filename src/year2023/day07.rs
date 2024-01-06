use crate::utils::parsers::*;
use std::cmp::Reverse;

fn val(j: usize, a: u8) -> usize {
    match a {
        b'T' => 10,
        b'J' => j,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        d => (d - b'0') as usize,
    }
}

fn parse(i: &str) -> [u8; 5] {
    i.as_bytes().try_into().unwrap()
}

fn solve(input: &str, j: usize) -> usize {
    let mut hands: Vec<_> = lines_iter(input, separated_pair(alphanumeric1, space1, usize))
        .map(|(cards, bid)| {
            let cards = parse(cards).map(|c| val(j, c));
            let mut cnt = [0; 15];
            cards.iter().for_each(|&c| cnt[c] += 1);
            let jokers = std::mem::take(&mut cnt[1]);
            cnt.sort_unstable_by_key(|&v| Reverse(v));
            cnt[0] += jokers;
            (cnt, cards, bid)
        })
        .collect();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 11)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1)
}
