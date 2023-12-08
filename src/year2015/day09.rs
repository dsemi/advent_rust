use crate::utils::parsers2::*;
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
