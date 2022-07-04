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

fn dfs(grid: &[Vec<u32>], vis: &mut [Vec<bool>], i: usize, j: usize) -> usize {
    if vis[i][j] || grid[i][j] == 9 {
        return 0;
    }
    vis[i][j] = true;
    neighbs(grid, &(i, j))
        .map(|(x, y)| dfs(grid, vis, x, y))
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
            let mut size = dfs(&grid, &mut visited, r, c);
            for v in basins.iter_mut() {
                if size > *v {
                    std::mem::swap(&mut size, v);
                }
            }
        }
    }
    basins.into_iter().product()
}
