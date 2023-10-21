use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list0 as s_list;
use nom::sequence::delimited;
use nom::{Err, IResult};
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
        map(delimited(tag("["), s_list(tag(","), parse), tag("]")), List),
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
        .flat_map(|seg| seg.lines().map(|line| parse(line).unwrap().1))
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .filter_map(|(i, pkts)| (pkts[0] < pkts[1]).then(|| i + 1))
        .sum()
}

pub fn part2(input: &str) -> Result<usize, Err<Error<&str>>> {
    let packets = input
        .split("\n\n")
        .flat_map(|seg| seg.lines().map(|line| parse(line).unwrap().1))
        .sorted_unstable()
        .collect::<Vec<_>>();
    Ok((packets.binary_search(&parse("[[2]]")?.1).unwrap_err() + 1)
        * (packets.binary_search(&parse("[[6]]")?.1).unwrap_err() + 2))
}
