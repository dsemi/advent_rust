use crate::utils::*;
use hashbrown::{HashMap, HashSet};
use itertools::iterate;
use std::iter::once;

fn solve<I: Iterator<Item = C<i32>>>(input: &str, pts: fn(C<i32>, C<i32>) -> I) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut groups = HashMap::new();
    let mut anti_nodes = HashSet::new();
    for (b, &v) in grid.idx_iter().filter(|&(_, &v)| v != b'.') {
        let e = groups.entry(v).or_insert_with(Vec::new);
        for &a in e.iter() {
            anti_nodes.extend(pts(a, b).take_while(|&i| grid.in_bounds(i)));
            anti_nodes.extend(pts(b, a).take_while(|&i| grid.in_bounds(i)));
        }
        e.push(b);
    }
    anti_nodes.len()
}

pub fn part1(input: &str) -> usize {
    solve(input, |a, b| once(a + a - b))
}

pub fn part2(input: &str) -> usize {
    solve(input, |a, b| iterate(a, move |v| v + a - b))
}
