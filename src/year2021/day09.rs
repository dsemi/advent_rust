use crate::utils::bfs;
use ahash::AHashSet;

type Coord = (usize, usize);

fn neighbs<'a>(grid: &'a [Vec<u32>], c: &(usize, usize)) -> impl Iterator<Item = Coord> + 'a {
    [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
    ]
    .into_iter()
    .filter_map(|p| grid.get(p.0)?.get(p.1).is_some().then(|| p))
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut risk = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if neighbs(&grid, &(r, c)).all(|p| grid[r][c] < grid[p.0][p.1]) {
                risk += 1 + grid[r][c]
            }
        }
    }
    risk
}

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visited = AHashSet::new();
    let mut basins = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let pt = (r, c);
            if !visited.contains(&pt) && grid[r][c] != 9 {
                basins.push(
                    bfs(pt, |p| neighbs(&grid, p).filter(|&(r, c)| grid[r][c] != 9))
                        .map(|(_, p)| visited.insert(p))
                        .count(),
                );
            }
        }
    }
    basins.sort_unstable();
    basins[basins.len() - 3..].iter().product()
}
