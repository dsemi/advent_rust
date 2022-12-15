use crate::utils::int;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use Packet::*;

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Lit(i32),
    List(Vec<Packet>),
}

fn parse(i: &str) -> IResult<&str, Packet> {
    alt((
        |i| int(i).map(|(i, n)| (i, Lit(n))),
        |i| {
            delimited(tag("["), separated_list0(tag(","), parse), tag("]"))(i)
                .map(|(i, ns)| (i, List(ns)))
        },
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
                .find(|&c| c != Equal)
                .unwrap_or_else(|| a.len().cmp(&b.len())),
            (Lit(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Lit(_)) => self.cmp(&List(vec![other.clone()])),
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, seg)| {
            let pkts = seg
                .lines()
                .map(|line| parse(line).unwrap().1)
                .collect::<Vec<_>>();
            (pkts[0] < pkts[1]).then(|| i + 1)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = input
        .split("\n\n")
        .flat_map(|seg| seg.lines().map(|line| parse(line).unwrap().1))
        .collect::<Vec<_>>();
    let a = parse("[[2]]").unwrap().1;
    let b = parse("[[6]]").unwrap().1;
    packets.push(a.clone());
    packets.push(b.clone());
    packets.sort_unstable();
    packets
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| (p == a || p == b).then(|| i + 1))
        .product()
}
