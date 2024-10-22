use crate::utils::parsers::*;
use crate::utils::C;
use ahash::AHashSet;
use std::iter::{once, repeat};

fn path(input: &str) -> impl Iterator<Item = C<i32>> + use<'_> {
    input
        .split(", ")
        .flat_map(|x| {
            let df = match x.chars().next().unwrap() {
                'R' => C(0, -1),
                'L' => C(0, 1),
                _ => panic!("Invalid dir {}", x),
            };
            let n = x[1..].usize();
            once(df).chain(repeat(C(1, 0)).take(n - 1))
        })
        .scan((C(0, 0), C(0, 1)), |state, x| {
            state.1 *= x;
            state.0 += state.1;
            Some(state.0)
        })
}

pub fn part1(input: &str) -> Option<i32> {
    path(input).last().map(|pos| pos.abs().sum())
}

pub fn part2(input: &str) -> Option<i32> {
    let mut s = AHashSet::new();
    path(input).find_map(|pos| (!s.insert(pos)).then(|| pos.abs().sum()))
}
