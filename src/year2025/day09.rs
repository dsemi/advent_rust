use crate::utils::parsers::*;
use crate::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::iter::once;

pub fn part1(input: &str) -> Option<u64> {
    lines(c(u64))
        .read(input)
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
}

pub fn part2(input: &str) -> Option<u64> {
    let coords = lines(c(u64)).read(input);
    let edges: Vec<_> = once(&coords[coords.len() - 1])
        .chain(coords.iter())
        .zip(coords.iter())
        .map(|(&c, &d)| (c.smol(d), c.swol(d)))
        .collect();
    coords
        .par_iter()
        .enumerate()
        .flat_map_iter(|(i, a)| coords.iter().skip(i + 1).map(|&b| (a.smol(b), a.swol(b))))
        .filter(|&(a, b)| {
            edges.iter().all(|(c, d)| d.0 <= a.0 || b.0 <= c.0 || d.1 <= a.1 || b.1 <= c.1)
        })
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
}
