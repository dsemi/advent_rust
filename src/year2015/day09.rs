use crate::utils::parsers::*;
use crate::utils::{held_karp, UniqueIdx};
use std::cmp::{max, min};

fn all_path_distances(input: &str, f: fn(usize, usize) -> usize) -> Option<usize> {
    let mut adj = Vec::new();
    let mut ui = UniqueIdx::new();
    let fake = ui.idx("fake");
    for line in input.lines() {
        let (k1, k2, v) = separated_triplet(alpha1, " to ", alpha1, " = ", usize).read(line);
        let n1 = ui.idx(k1);
        let n2 = ui.idx(k2);
        if max(n1, n2) >= adj.len() {
            adj.resize_with(max(n1, n2) + 1, Vec::new);
        }
        if n1 >= adj[n2].len() {
            adj[n2].resize(n1 + 1, 0);
        }
        if n2 >= adj[n1].len() {
            adj[n1].resize(n2 + 1, 0);
        }
        adj[n1][n2] = v;
        adj[n2][n1] = v;
    }
    let len = adj.len();
    adj[fake].extend(vec![0; len]);
    held_karp(&adj, f)
}

pub fn part1(input: &str) -> Option<usize> {
    all_path_distances(input, min)
}

pub fn part2(input: &str) -> Option<usize> {
    all_path_distances(input, max)
}
