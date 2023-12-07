use crate::utils::parsers::*;
use Hand::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn freq(i: &[u8]) -> u64 {
    i.iter().fold(0, |acc, b| acc + (1 << (3 * b)))
}

fn hand(bits: u64) -> Hand {
    let ones = bits.count_ones();
    match (ones, bits & (bits << 2) > 0, bits & (bits << 1) > 0) {
        (2, true, _) => FiveOfAKind,
        (2, false, _) => FourOfAKind,
        (3, _, true) => FullHouse,
        (3, _, false) => TwoPair,
        (4, _, true) => ThreeOfAKind,
        (4, _, false) => OnePair,
        (_, _, _) => HighCard,
    }
}

fn val(a: u8) -> u8 {
    match a {
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        d => d - b'0',
    }
}

fn parse(i: &str) -> [u8; 5] {
    i.as_bytes().try_into().unwrap()
}

fn solve(input: &str, hand: fn(&[u8]) -> Hand, val: fn(u8) -> u8) -> usize {
    let mut hands: Vec<_> = lines_iter(input, separated_pair(alphanumeric1, space1, usize))
        .map(|(cards, bid)| {
            let cards = parse(cards).map(val);
            (hand(&cards), cards, bid)
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
    solve(input, |s| hand(freq(s)), val)
}

fn hand2(i: &[u8]) -> Hand {
    let mut bits = freq(i);
    let jokers = bits & 0b111;
    bits >>= 3;
    let ones = bits.count_ones();
    match (jokers, ones, bits & (bits << 1) > 0) {
        (0, _, _) => hand(bits),
        (1, 1, _) => FiveOfAKind,
        (1, 2, _) => FullHouse,
        (1, 3, true) => FourOfAKind,
        (1, 3, false) => ThreeOfAKind,
        (1, 4, _) => OnePair,
        (2, 2, true) => FiveOfAKind,
        (2, 2, false) => FourOfAKind,
        (2, 3, _) => ThreeOfAKind,
        (3, 1, _) => FiveOfAKind,
        (3, 2, _) => FourOfAKind,
        (_, _, _) => FiveOfAKind,
    }
}

fn val2(a: u8) -> u8 {
    match a {
        b'J' => 0,
        b => val(b),
    }
}

pub fn part2(input: &str) -> usize {
    solve(input, hand2, val2)
}
