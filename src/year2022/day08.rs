use crate::utils::*;
use itertools::iterate;

fn trees(input: &str) -> impl Iterator<Item = (bool, u32)> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let size = grid.len();
    (0..grid.len() * grid[0].len()).map(move |i| {
        let (r, c) = (i / size, i % size);
        [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)].into_iter().fold(
            (false, 1),
            |(visible_from_edge, scenic_score), d| {
                let mut cnt = 0;
                let reaches_edge = iterate(C(r as i32, c as i32) + d, |x| x + d)
                    .scan((), |_, p| grid.get_cell(p))
                    .inspect(|_| cnt += 1)
                    .all(|&x| x < grid[r][c]);
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
