use crate::utils::bfs;
use ahash::AHashSet;

type Coord = (usize, usize);

fn neighbs<'a>(grid: &'a [Vec<u32>], c: &Coord) -> impl Iterator<Item = Coord> + 'a {
    [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
    ]
    .into_iter()
    .filter_map(|p| grid.get(p.0)?.get(p.1).map(|_| p))
}

fn lows(grid: &[Vec<u32>]) -> impl Iterator<Item = Coord> + '_ {
    (0..grid.len()).flat_map(move |r| {
        (0..grid[r].len()).filter_map(move |c| {
            neighbs(grid, &(r, c))
                .all(|p| grid[r][c] < grid[p.0][p.1])
                .then(|| (r, c))
        })
    })
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    lows(&grid).map(|(r, c)| 1 + grid[r][c]).sum()
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visited = AHashSet::new();
    let mut basins = lows(&grid)
        .map(|pt| {
            bfs(pt, |p| neighbs(&grid, p).filter(|&(r, c)| grid[r][c] != 9))
                .take_while(|(_, p)| visited.insert(*p))
                .count()
        })
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins[basins.len() - 3..].iter().product()
}
