use crate::utils::{held_karp, UniqueIdx};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::combinator::{map, value};
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::max;

fn parse(i: &str) -> IResult<&str, (&str, i32, &str)> {
    map(
        tuple((
            alpha1,
            tag(" would "),
            alt((value(1, tag("gain ")), value(-1, tag("lose ")))),
            i32,
            tag(" happiness units by sitting next to "),
            alpha1,
        )),
        |(p1, _, sgn, n, _, p2)| (p1, sgn * n, p2),
    )(i)
}

fn parse_happiness(input: &str) -> Vec<Vec<i32>> {
    let mut ui = UniqueIdx::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l = (input.lines().count() as f32).sqrt() as usize + 1;
    for _ in 0..l {
        result.push(vec![0; l]);
    }
    for line in input.lines() {
        let (p1, n, p2) = parse(line).unwrap().1;
        result[ui.idx(p1)][ui.idx(p2)] += n;
        result[ui.idx(p2)][ui.idx(p1)] += n;
    }
    result
}

pub fn part1(input: &str) -> Option<i32> {
    held_karp(&parse_happiness(input), max)
}

pub fn part2(input: &str) -> Option<i32> {
    let mut adj = parse_happiness(input);
    adj.iter_mut().for_each(|row| row.push(0));
    adj.push(vec![0; adj.len()]);
    held_karp(&adj, max)
}
