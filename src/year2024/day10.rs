use crate::utils::*;

fn dfs(grid: &Grid<u8, i32>, (pt, v): (C<i32>, u8), f: &mut impl FnMut(C<i32>) -> bool) -> usize {
    if !f(pt) {
        return 0;
    }
    if v == b'9' {
        return 1;
    }
    [C(1, 0), C(-1, 0), C(0, 1), C(0, -1)]
        .into_iter()
        .filter_map(|d| Some((pt + d, *grid.get(pt + d)?)))
        .filter(|&(_, v2)| v2 == v + 1)
        .map(|p| dfs(grid, p, f))
        .sum()
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    grid.positions(|&v| v == b'0')
        .map(|start| {
            let mut unseen = grid.same_size_with(true);
            dfs(&grid, (start, grid[start]), &mut |p| std::mem::replace(&mut unseen[p], false))
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    grid.positions(|&v| v == b'0')
        .map(|start| dfs(&grid, (start, grid[start]), &mut |_| true))
        .sum()
}
