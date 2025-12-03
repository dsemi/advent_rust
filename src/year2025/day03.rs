use crate::utils::*;

fn max_jolt(row: &[u64], ns: usize) -> u64 {
    let mut start = 0;
    (row.len() - ns + 1..=row.len()).fold(0, |curr, end| {
        start = (start..end).rev().max_by_key(|&i| row[i]).unwrap() + 1;
        10 * curr + row[start - 1]
    })
}

pub fn part1(input: &str) -> u64 {
    let grid: Grid<u64> = Grid::ints(input.bytes());
    grid.rows().map(|row| max_jolt(row, 2)).sum()
}

pub fn part2(input: &str) -> u64 {
    let grid: Grid<u64> = Grid::ints(input.bytes());
    grid.rows().map(|row| max_jolt(row, 12)).sum()
}
