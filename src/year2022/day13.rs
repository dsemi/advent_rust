use crate::utils::parsers::*;
use Packet::*;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Lit(i32),
    List(Vec<Packet>),
}

fn parse(i: &mut &str) -> ModalResult<Packet> {
    alt((i32.map(Lit), delimited('[', list(parse), ']').map(List))).parse_next(i)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Lit(a), Lit(b)) => a.cmp(b),
            (List(a), List(b)) => a
                .iter()
                .zip(b.iter())
                .map(|(x, y)| x.cmp(y))
                .find(|&c| c != Ordering::Equal)
                .unwrap_or_else(|| a.len().cmp(&b.len())),
            (Lit(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Lit(_)) => self.cmp(&List(vec![other.clone()])),
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(|seg| lines_iter(seg, parse))
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .filter_map(|(i, pkts)| (pkts[0] < pkts[1]).then_some(i + 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let packets = input
        .split("\n\n")
        .flat_map(|seg| lines_iter(seg, parse))
        .sorted_unstable()
        .collect::<Vec<_>>();
    let a = parse.read("[[2]]");
    let b = parse.read("[[6]]");
    (packets.binary_search(&a).unwrap_err() + 1) * (packets.binary_search(&b).unwrap_err() + 2)
}
