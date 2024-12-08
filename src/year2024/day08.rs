use crate::utils::*;
use ahash::{AHashMap, AHashSet};
use itertools::iterate;
use std::iter::once;

fn solve<I, I2>(input: &str, pts: fn(C<i32>, C<i32>) -> (I, I2)) -> usize
where
    I: Iterator<Item = C<i32>>,
    I2: Iterator<Item = C<i32>>,
{
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut groups = AHashMap::new();
    let mut anti_nodes = AHashSet::new();
    for (b, &v) in grid.idx_iter().filter(|&(_, &v)| v != b'.') {
        let e = groups.entry(v).or_insert_with(Vec::new);
        for &a in e.iter() {
            let (a_s, b_s) = pts(a, b);
            anti_nodes.extend(a_s.take_while(|&i| grid.in_bounds(i)));
            anti_nodes.extend(b_s.take_while(|&i| grid.in_bounds(i)));
        }
        e.push(b);
    }
    anti_nodes.len()
}

pub fn part1(input: &str) -> usize {
    solve(input, |a, b| (once(a + a - b), once(b + b - a)))
}

pub fn part2(input: &str) -> usize {
    solve(input, |a, b| (iterate(a, move |v| v + a - b), iterate(b, move |v| v + b - a)))
}
