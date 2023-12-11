use crate::utils::positions;

fn solve(input: &str, exp: usize) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let vert_dists: Vec<_> = (0..grid.len())
        .map(|r| grid[r].iter().all(|&c| c == '.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let horz_dists: Vec<_> = (0..grid[0].len())
        .map(|c| grid.iter().all(|row| row[c] == '.') as usize * (exp - 1) + 1)
        .scan(0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let mut galaxies = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == '#' {
                galaxies.push((r, c));
            }
        }
    }
    let galaxies = positions(&grid, '#');
    let mut dists = 0;
    for (i, &(r0, c0)) in galaxies.iter().enumerate() {
        for &(r1, c1) in galaxies.iter().skip(i + 1) {
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
