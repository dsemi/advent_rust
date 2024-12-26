use crate::utils::*;
use hashbrown::HashSet;

pub fn part1(input: &str) -> usize {
    let mut ui = UniqueIdx::new();
    let pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a, b, ui.idx(a), ui.idx(b))
        })
        .collect();
    let mut adj = vec![vec![false; ui.len()]; ui.len()];
    let mut ts = vec![false; ui.len()];
    for (a, b, ai, bi) in pairs {
        adj[ai][bi] = true;
        adj[bi][ai] = true;
        if a.as_bytes()[0] == b't' {
            ts[ai] = true;
        }
        if b.as_bytes()[0] == b't' {
            ts[bi] = true;
        }
    }
    let mut seen = HashSet::new();
    for (a, row) in adj.iter().enumerate() {
        for (b, _) in row.iter().enumerate().skip(a + 1).filter(|x| *x.1) {
            for (c, _) in row.iter().enumerate().skip(b + 1).filter(|x| *x.1) {
                if adj[b][c] {
                    seen.insert((a, b, c));
                }
            }
        }
    }
    println!("{}", ui.len());
    seen.into_iter().filter(|&(a, b, c)| ts[a] || ts[b] || ts[c]).count()
}

pub fn part2(input: &str) -> String {
    let mut ui = UniqueIdx::new();
    let mut ks = vec![];
    let pairs: Vec<_> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (ui.idx_with(a, || ks.push(a)), ui.idx_with(b, || ks.push(b)))
        })
        .collect();
    let mut adj = vec![vec![false; ui.len()]; ui.len()];
    for (a, b) in pairs {
        adj[a][b] = true;
        adj[b][a] = true;
    }
    let mut curr = vec![];
    for i in 0..adj.len() {
        let mut next = vec![i];
        for j in 0..adj.len() {
            if next.iter().all(|&i| adj[i][j]) {
                next.push(j);
            }
        }
        if next.len() > curr.len() {
            curr = next;
        }
    }
    println!("{}", curr.len());
    let mut comps: Vec<_> = curr.into_iter().map(|k| ks[k]).collect();
    comps.sort_unstable();
    comps.join(",")
}
