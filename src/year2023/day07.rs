use crate::utils::parsers::*;
use Hand::*;

enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn add_card(&mut self, count: u8) {
        match (&self, count) {
            (HighCard, 2) => *self = Pair,
            (Pair, 2) => *self = TwoPair,
            (Pair, 3) => *self = ThreeOfAKind,
            (ThreeOfAKind, 2) => *self = FullHouse,
            (ThreeOfAKind, 4) => *self = FourOfAKind,
            (FourOfAKind, 5) => *self = FiveOfAKind,
            _ => (),
        }
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

fn solve(input: &str, j: u8) -> usize {
    let mut hands: Vec<_> = lines_iter(input, separated_pair(alphanumeric1, space1, u64))
        .map(|(cards, bid)| {
            let mut cnt = [0_u8; 15];
            let mut ord = [0_u8; 8];
            let mut hand = HighCard;
            cards.bytes().enumerate().for_each(|(i, c)| {
                let v = val(j, c);
                ord[i + 1] = v;
                cnt[v as usize] += 1;
                if v > 1 {
                    hand.add_card(cnt[v as usize]);
                }
            });
            let jokers = std::mem::take(&mut cnt[1]);
            let most_freq_card = cnt.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
            for _ in 0..jokers {
                cnt[most_freq_card] += 1;
                hand.add_card(cnt[most_freq_card]);
            }
            ord[0] = hand as u8;
            u64::from_be_bytes(ord) + bid
        })
        .collect();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * (hand & u16::MAX as u64) as usize)
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 11)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1)
}
