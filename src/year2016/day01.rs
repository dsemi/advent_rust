use crate::utils::C;
use ahash::AHashSet;
use std::iter;

fn path(input: &str) -> impl Iterator<Item = C<i32>> + '_ {
    input
        .split(", ")
        .flat_map(|x| {
            let df = match x.chars().next().unwrap() {
                'R' => C(0, -1),
                'L' => C(0, 1),
                _ => panic!("Invalid dir {}", x),
            };
            let n: usize = x[1..].parse().unwrap();
            iter::once(df).chain(iter::repeat(C(1, 0)).take(n - 1))
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
