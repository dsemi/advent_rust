use crate::utils::parsers::*;
use crate::utils::*;

fn neighbors(g: &Grid<usize, i32>, t: usize, p: C<i32>) -> impl Iterator<Item = C<i32>> + use<'_> {
    [C(0, 1), C(0, -1), C(1, 0), C(-1, 0)]
        .into_iter()
        .map(move |d| p + d)
        .filter(move |&p| g.in_bounds(p) && t < g[p])
}

fn solve(grid: &Grid<usize, i32>, t: usize) -> Option<usize> {
    bfs(C(0, 0), |&pos| neighbors(grid, t, pos))
        .find_map(|(steps, pos)| (pos == C(70, 70)).then_some(steps))
}

pub fn part1(input: &str) -> Option<usize> {
    let mut grid = Grid::new_with(71, 71, usize::MAX);
    lines_iter(input, coord(i32)).enumerate().for_each(|(t, (c, r))| grid[(r, c)] = t);
    solve(&grid, 1024)
}

pub fn part2(input: &str) -> (i32, i32) {
    // Might be able to speed up with union-find connecting top-right border to bottom-left.
    let mut grid = Grid::new_with(71, 71, usize::MAX);
    let bytes = lines(coord(i32)).read(input);
    bytes.iter().enumerate().for_each(|(t, &(c, r))| grid[(r, c)] = t);
    bytes[partition_point(0, bytes.len() - 1, |&t| solve(&grid, t).is_some())]
}
