use crate::utils::parsers::*;
use itertools::Itertools;
use std::cmp::Ordering;
use Packet::*;

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Lit(i32),
    List(Vec<Packet>),
}

fn parse(i: &str) -> IResult<&str, Packet> {
    alt((
        map(i32, Lit),
        map(delimited(tag("["), list(parse), tag("]")), List),
    ))(i)
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
    let a = parse("[[2]]").unwrap().1;
    let b = parse("[[6]]").unwrap().1;
    (packets.binary_search(&a).unwrap_err() + 1) * (packets.binary_search(&b).unwrap_err() + 2)
}
