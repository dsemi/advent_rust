use crate::utils::*;
use std::collections::VecDeque;

fn parse(input: &str) -> (Vec<Vec<(usize, usize)>>, usize) {
    let mut adj = Vec::new();
    let mut edges = 0;
    let mut ui = UniqueIdx::new();
    let mut edge_ui = UniqueIdx::new();
    for line in input.lines() {
        let (k1, ks) = line.split_once(": ").unwrap();
        let n1 = ui.idx(k1);
        if n1 >= adj.len() {
            adj.resize_with(n1 + 1, Vec::new);
        }
        for k2 in ks.split_whitespace() {
            let n2 = ui.idx(k2);
            let edge = edge_ui.idx((n1.min(n2), n1.max(n2)));
            if edge >= edges {
                edges += 1;
            }
            adj[n1].push((n2, edge));
            if n2 >= adj.len() {
                adj.resize_with(n2 + 1, Vec::new);
            }
            adj[n2].push((n1, edge));
        }
    }
    (adj, edges)
}

fn neighbors(adj: &[Vec<(usize, usize)>], curr: usize) -> impl Iterator<Item = usize> + '_ {
    adj[curr].iter().map(|x| x.0)
}

pub fn part1(input: &str) -> usize {
    let (adj, edge_count) = parse(input);
    // Arbitrary start.
    let start = bfs(0, |n| neighbors(&adj, *n)).last().unwrap().1;
    let end = bfs(start, |n| neighbors(&adj, *n)).last().unwrap().1;
    let mut flowed = vec![false; edge_count];
    let mut cnt = 0;
    let mut path = Vec::new();
    let mut frontier = VecDeque::new();
    for _ in 0..4 {
        frontier.push_back((start, None));
        let mut visited = vec![false; adj.len()];
        visited[start] = true;
        cnt = 0;
        while let Some((curr, mut prev)) = frontier.pop_front() {
            cnt += 1;
            if curr == end {
                while let Some(idx) = prev {
                    let (prev_opt, edge) = path[idx];
                    flowed[edge] = true;
                    prev = prev_opt;
                }
                break;
            }
            for &(node, edge) in &adj[curr] {
                if !visited[node] && !flowed[edge] {
                    visited[node] = true;
                    frontier.push_back((node, Some(path.len())));
                    path.push((prev, edge));
                }
            }
        }
        frontier.clear();
        path.clear();
    }
    cnt * (adj.len() - cnt)
}

pub fn part2(_input: &str) -> &str {
    " "
}
