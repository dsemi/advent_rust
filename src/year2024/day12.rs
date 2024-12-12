use crate::utils::*;
use std::array::from_fn;

const ADJ: [C<i32>; 4] = [C(-1, 0), C(0, -1), C(0, 1), C(1, 0)];

fn fill(comps: &mut Grid<usize, i32>, n: usize, grid: &Grid<u8, i32>, p: C<i32>, v: u8) {
    if std::mem::replace(&mut comps[p], n) == n {
        return;
    }
    ADJ.iter()
        .map(|d| p + d)
        .filter(|&adj| *grid.get(adj).unwrap_or(&b'.') == v)
        .for_each(|adj| fill(comps, n, grid, adj, v));
}

fn solve(input: &str, f: impl Fn(&Grid<usize, i32>, C<i32>, usize) -> usize) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut components = grid.same_size_with(0);
    let mut n = 1;
    for (i, &v) in grid.idx_iter() {
        if components[i] == 0 {
            fill(&mut components, replace_with(&mut n, |n| n + 1), &grid, i, v);
        }
    }
    let mut avs = vec![(0, 0); n];
    for (i, &v) in components.idx_iter() {
        avs[v].0 += 1;
        avs[v].1 += f(&components, i, v);
    }
    avs.into_iter().map(|(a, v)| a * v).sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, |components, i, v| {
        ADJ.iter().map(|d| i + d).filter(|&adj| *components.get(adj).unwrap_or(&0) != v).count()
    })
}

const DIAG: [C<i32>; 8] =
    [C(-1, -1), C(-1, 0), C(-1, 1), C(0, -1), C(0, 1), C(1, -1), C(1, 0), C(1, 1)];

// Make const when const from_fn: https://github.com/rust-lang/rust/issues/109341
fn corners() -> [usize; 256] {
    from_fn(|adjs| {
        let [ul, u, ur, l, r, dl, d, dr] = from_fn(|i| adjs & (1 << i) != 0);
        let ul = (!u && !l || u && l && !ul) as usize;
        let ur = (!u && !r || u && r && !ur) as usize;
        let dl = (!d && !l || d && l && !dl) as usize;
        let dr = (!d && !r || d && r && !dr) as usize;
        ul + ur + dl + dr
    })
}

pub fn part2(input: &str) -> usize {
    let corners = corners();
    solve(input, |components, i, v| {
        corners[DIAG
            .iter()
            .map(|d| i + d)
            .enumerate()
            .filter_map(|(i, adj)| (*components.get(adj).unwrap_or(&0) == v).then_some(1 << i))
            .fold(0, |acc, b| acc | b)]
    })
}
