use crate::utils::parsers::*;
use crate::utils::*;
use std::borrow::Borrow;
use streaming_iterator::StreamingIterator;

fn valid<T: Borrow<i8>>(ns: &[T]) -> bool {
    let sgn = (ns[0].borrow() - ns[1].borrow()).signum();
    ns.array_windows()
        .map(|[a, b]| a.borrow() - b.borrow())
        .all(|d| d.signum() == sgn && (1..=3).contains(&d.abs()))
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let ns: Vec<_> = spaced(i8).read(line);
            valid(&ns)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let ns: Vec<_> = spaced(i8).read(line);
            valid(&ns) || ns.combinations(ns.len() - 1).any(valid)
        })
        .count()
}
