use crate::utils::C;

fn neighbs<'a>(grid: &'a [Vec<u32>], c: &C<usize>) -> impl Iterator<Item = C<usize>> + 'a {
    [
        C(c.0 - 1, c.1),
        C(c.0 + 1, c.1),
        C(c.0, c.1 - 1),
        C(c.0, c.1 + 1),
    ]
    .into_iter()
    .filter_map(|p| grid.get(p.0)?.get(p.1).map(|_| p))
}

fn lows(grid: &[Vec<u32>]) -> impl Iterator<Item = C<usize>> + '_ {
    (0..grid.len()).flat_map(move |r| {
        (0..grid[r].len()).filter_map(move |c| {
            neighbs(grid, &C(r, c))
                .all(|p| grid[r][c] < grid[p])
                .then(|| C(r, c))
        })
    })
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    lows(&grid).map(|p| 1 + grid[p]).sum()
}

fn dfs(grid: &[Vec<u32>], vis: &mut [Vec<bool>], ij: C<usize>) -> usize {
    if vis[ij] || grid[ij] == 9 {
        return 0;
    }
    vis[ij] = true;
    neighbs(grid, &ij)
        .map(|xy| dfs(grid, vis, xy))
        .sum::<usize>()
        + 1
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut basins = [0; 3];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let mut size = dfs(&grid, &mut visited, C(r, c));
            for v in basins.iter_mut() {
                if size > *v {
                    std::mem::swap(&mut size, v);
                }
            }
        }
    }
    basins.into_iter().product()
}
