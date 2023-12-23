use crate::utils::*;
use itertools::iterate;

fn trees(input: &str) -> impl Iterator<Item = (bool, u32)> {
    let grid: Grid<u32, i32> = Grid::ints(input.bytes());
    assert!(grid.rows == grid.cols);
    grid.idxs().map(move |i| {
        [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)].into_iter().fold(
            (false, 1),
            |(visible_from_edge, scenic_score), d| {
                let mut cnt = 0;
                let reaches_edge = iterate(i + d, |x| x + d)
                    .scan((), |_, p| grid.get(p))
                    .inspect(|_| cnt += 1)
                    .all(|&x| x < grid[i]);
                (visible_from_edge | reaches_edge, scenic_score * cnt)
            },
        )
    })
}

pub fn part1(input: &str) -> usize {
    trees(input).filter(|t| t.0).count()
}

pub fn part2(input: &str) -> Option<u32> {
    trees(input).map(|t| t.1).max()
}
