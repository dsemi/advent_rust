use crate::utils::parsers::*;
use crate::utils::*;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> u32 {
    let mut pts: UnionFind<_> = input.lines().map(|line| c3(u64).read(line)).collect();
    (0..pts.len())
        .tuple_combinations()
        .map(|(i, j)| (pts[i].dist_sq(&pts[j]), i, j))
        .k_smallest(1000)
        .for_each(|(_, i, j)| std::mem::drop(pts.union(i, j)));
    let mut comps = vec![0; pts.len()];
    (0..pts.len()).for_each(|i| comps[pts.find(i)] += 1);
    comps.into_iter().k_largest(3).product()
}

pub fn part2(input: &str) -> u64 {
    let mut pts: UnionFind<_> = input.lines().map(|line| c3(u64).read(line)).collect();
    let pairs: BinaryHeap<_> = (0..pts.len())
        .tuple_combinations()
        .map(|(i, j)| Reverse((pts[i].dist_sq(&pts[j]), i, j)))
        .collect();
    let mut connections = 0;
    for Reverse((_, i, j)) in pairs.into_iter_sorted() {
        if pts.union(i, j) {
            connections += 1;
            if connections == pts.len() - 1 {
                return pts[i].0 * pts[j].0;
            }
        }
    }
    unreachable!()
}
