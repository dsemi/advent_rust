use crate::utils::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn part1(input: &str) -> usize {
    let mut ui = UniqueIdx::new();
    let mut edges = Vec::new();
    let mut union_find = UnionFind::new();
    for line in input.lines() {
        let (k, ks) = line.split_once(": ").unwrap();
        let idx = ui.idx(k);
        if idx >= union_find.len() {
            union_find.push(idx);
        }
        for k in ks.split_whitespace() {
            let i = ui.idx(k);
            if i >= union_find.len() {
                union_find.push(i);
            }
            edges.push((idx, i));
        }
    }
    let mut rng = thread_rng();
    let mut min_cut = vec![];
    while min_cut.len() != 3 {
        let mut uf = union_find.clone();
        edges.shuffle(&mut rng);
        let mut components = uf.len();
        for &(u, v) in edges.iter() {
            let (pu, pv) = (uf.find(u), uf.find(v));
            if pu != pv {
                uf.union(pu, pv);
                components -= 1;
                if components == 2 {
                    break;
                }
            }
        }
        if components > 2 {
            continue;
        }
        min_cut = edges
            .iter()
            .copied()
            .filter(|&(a, b)| uf.find(a) != uf.find(b))
            .collect();
    }
    let mut uf = union_find.clone();
    for &(a, b) in edges.iter() {
        if !min_cut.contains(&(a, b)) {
            uf.union(a, b);
        }
    }
    let cnts = (0..ui.len()).into_iter().map(|n| uf.find(n)).counts();
    assert!(cnts.len() == 2);
    cnts.into_values().product()
}

pub fn part2(_input: &str) -> &str {
    " "
}
