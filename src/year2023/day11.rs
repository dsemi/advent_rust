use crate::utils::*;

fn solve(input: &str, exp: usize) -> usize {
    let grid: Grid<_> = input.bytes().collect();
    let vert_dists: Vec<_> = (0..grid.rows)
        .map(|r| grid.row(r).all(|&v| v == b'.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let horz_dists: Vec<_> = (0..grid.cols)
        .map(|c| grid.col(c).all(|&v| v == b'.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let galaxies: Vec<_> = grid.positions(|&v| v == b'#').collect();
    let mut dists = 0;
    for (i, &C(r0, c0)) in galaxies.iter().enumerate() {
        for &C(r1, c1) in galaxies.iter().skip(i + 1) {
            dists += vert_dists[r0.max(r1)] - vert_dists[r0.min(r1)];
            dists += horz_dists[c0.max(c1)] - horz_dists[c0.min(c1)];
        }
    }
    dists
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1000000)
}
