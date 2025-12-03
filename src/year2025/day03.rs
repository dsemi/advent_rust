use crate::utils::*;

fn max_jolt(mut row: &[u64], ns: usize) -> u64 {
    let mut curr = 0;
    (0..ns).rev().for_each(|off| {
        let (idx, n) = row[..row.len() - off].iter().enumerate().rev().max_by_key(|x| x.1).unwrap();
        row = &row[idx + 1..];
        curr = 10 * curr + n;
    });
    curr
}

pub fn part1(input: &str) -> u64 {
    let grid: Grid<u64> = Grid::ints(input.bytes());
    grid.rows().map(|row| max_jolt(row, 2)).sum()
}

pub fn part2(input: &str) -> u64 {
    let grid: Grid<u64> = Grid::ints(input.bytes());
    grid.rows().map(|row| max_jolt(row, 12)).sum()
}
