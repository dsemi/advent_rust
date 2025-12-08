use crate::utils::parsers::*;
use crate::utils::*;
use itertools::Itertools;

fn parse(input: &str) -> (Vec<(usize, usize)>, UnionFind<C3<i64>>) {
    let c = lines(c3(i64)).read(input);
    let mut dists: Vec<_> = c
        .iter()
        .enumerate()
        .flat_map(|(i, a)| c.iter().enumerate().skip(i + 1).map(move |(j, b)| (a.dist_sq(b), i, j)))
        .collect();
    dists.sort_unstable();
    (dists.into_iter().map(|(_, i, j)| (i, j)).collect(), c.into_iter().collect())
}

pub fn part1(input: &str) -> u32 {
    let (dists, mut pts) = parse(input);
    dists.iter().take(1000).for_each(|&(i, j)| pts.union(i, j));
    let mut comps = vec![0; pts.len()];
    (0..pts.len()).for_each(|i| comps[pts.find(i)] += 1);
    comps.into_iter().k_largest(3).product()
}

pub fn part2(input: &str) -> i64 {
    let (dists, mut pts) = parse(input);
    dists
        .into_iter()
        .try_for_each(|(i, j)| {
            pts.union(i, j);
            (pts.ncomponents() > 1).ok_or(pts[i].0 * pts[j].0)
        })
        .unwrap_err()
}
