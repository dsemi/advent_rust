use crate::utils::parsers::*;
use crate::utils::*;
use itertools::Itertools;
use std::ops::{Add, Mul};

#[derive(Clone)]
struct Col(fn(u64, u64) -> u64, u64);

fn parse(input: &str) -> (Vec<String>, Vec<Col>) {
    let mut lines = lines(repeat(.., none_of('\n'))).read(input);
    let ops: String = lines.pop().unwrap();
    let cols =
        spaced(alt(('+'.value(Col(u64::add, 0)), '*'.value(Col(u64::mul, 1))))).read(ops.as_str());
    (lines, cols)
}

pub fn part1(input: &str) -> u64 {
    let (lines, mut cols) = parse(input);
    lines.into_iter().for_each(|line| {
        cols.iter_mut()
            .zip(line.split_whitespace().map(|x| x.u64()))
            .for_each(|(c, n)| c.1 = c.0(c.1, n))
    });
    cols.into_iter().map(|c| c.1).sum()
}

pub fn part2(input: &str) -> u64 {
    let (lines, mut cols) = parse(input);
    let transposed =
        transpose_str(&lines).into_iter().map(|line| line.trim().to_owned()).join("\n");
    cols.iter_mut()
        .zip(transposed.split("\n\n"))
        .for_each(|(c, ns)| ns.lines().for_each(|n| c.1 = c.0(c.1, n.u64())));
    cols.into_iter().map(|c| c.1).sum()
}
