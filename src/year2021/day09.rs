use crate::utils::*;

fn neighbs<'a>(grid: &'a Grid<u32>, c: &C<usize>) -> impl Iterator<Item = C<usize>> + 'a {
    [
        C(c.0 - 1, c.1),
        C(c.0 + 1, c.1),
        C(c.0, c.1 - 1),
        C(c.0, c.1 + 1),
    ]
    .into_iter()
    .filter_map(|p| grid.get(p).map(|_| p))
}

fn lows(grid: &Grid<u32>) -> impl Iterator<Item = C<usize>> + '_ {
    (0..grid.rows).flat_map(move |r| {
        (0..grid.cols).filter_map(move |c| {
            neighbs(grid, &C(r, c))
                .all(|p| grid[(r, c)] < grid[p])
                .then_some(C(r, c))
        })
    })
}

pub fn part1(input: &str) -> u32 {
    let grid = Grid::ints(input.bytes());
    lows(&grid).map(|p| 1 + grid[p]).sum()
}

fn dfs(grid: &Grid<u32>, vis: &mut Grid<bool>, ij: C<usize>) -> usize {
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
    let grid = Grid::ints(input.bytes());
    let mut visited = grid.same_size();
    let mut basins = [0; 3];
    for r in 0..grid.rows {
        for c in 0..grid.cols {
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
