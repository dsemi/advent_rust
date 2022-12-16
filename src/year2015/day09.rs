use crate::utils::UniqueIdx;
use itertools::Itertools;
use std::cmp::max;

fn all_path_distances(input: &str) -> impl Iterator<Item = usize> {
    let mut adj = Vec::new();
    let mut ui = UniqueIdx::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let (k1, k2, v) = (parts[0], parts[2], parts[4].parse().unwrap());
        let n1 = ui.idx(k1);
        let n2 = ui.idx(k2);
        while max(n1, n2) >= adj.len() {
            adj.push(Vec::new());
        }
        while n1 >= adj[n2].len() {
            adj[n2].push(0);
        }
        while n2 >= adj[n1].len() {
            adj[n1].push(0);
        }
        adj[n1][n2] = v;
        adj[n2][n1] = v;
    }
    (0..adj.len())
        .permutations(adj.len())
        .map(move |perm| perm.windows(2).map(|p| adj[p[0]][p[1]]).sum())
}

pub fn part1(input: &str) -> Option<usize> {
    all_path_distances(input).min()
}

pub fn part2(input: &str) -> Option<usize> {
    all_path_distances(input).max()
}
