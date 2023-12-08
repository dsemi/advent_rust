use crate::utils::parsers2::*;
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

const FOUR: u64 = 0x124924924924;
const TWO: u64 = FOUR >> 1;

fn hand(cards: &[u8]) -> Hand {
    let mut bits = cards.iter().fold(0, |acc, b| acc + (1 << (3 * b)));
    let jokers = bits & 0b111;
    bits >>= 3;
    if jokers >= 4 {
        return FiveOfAKind;
    }
    if bits & FOUR > 0 {
        bits += jokers << ((bits & FOUR).trailing_zeros() - 2);
    } else if bits & (bits << 1) & TWO > 0 {
        bits += jokers << ((bits & (bits << 1) & TWO).trailing_zeros() - 1);
    } else if bits & TWO > 0 {
        bits += jokers << ((bits & TWO).trailing_zeros() - 1);
    } else {
        bits += jokers << bits.trailing_zeros();
    }
    if bits & (bits << 2) & FOUR > 0 {
        FiveOfAKind
    } else if bits & FOUR > 0 {
        FourOfAKind
    } else if bits & (bits << 1) & TWO > 0 {
        if (bits & TWO).count_ones() == 2 {
            FullHouse
        } else {
            ThreeOfAKind
        }
    } else if bits & TWO > 0 {
        if (bits & TWO).count_ones() == 2 {
            TwoPair
        } else {
            OnePair
        }
    } else {
        HighCard
    }
}

fn val(j: u8, a: u8) -> u8 {
    match a {
        b'T' => 10,
        b'J' => j,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        d => d - b'0',
    }
}

fn parse(i: &str) -> [u8; 5] {
    i.as_bytes().try_into().unwrap()
}

fn solve(input: &str, j: u8) -> usize {
    let mut hands: Vec<_> = lines_iter(input, separated_pair(alphanumeric1, space1, usize))
        .map(|(cards, bid)| {
            let cards = parse(cards).map(|c| val(j, c));
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
    solve(input, 11)
}

pub fn part2(input: &str) -> usize {
    solve(input, 0)
}
