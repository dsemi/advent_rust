use crate::utils::*;

fn dfs(grid: &mut Grid<u8>, pos: C<usize>) -> (u64, bool) {
    match std::mem::replace(grid.get_mut(pos).unwrap_or(&mut 0), 0) {
        0 => (0, false),
        b'^' => {
            let (l, r) = (dfs(grid, pos - C(0, 1)), dfs(grid, pos + C(0, 1)));
            (u64::from(l.1 || r.1) + l.0 + r.0, true)
        }
        _ => (dfs(grid, pos + C(1, 0)).0, true),
    }
}

pub fn part1(input: &str) -> u64 {
    let mut grid: Grid<_> = input.bytes().collect();
    let start = C(0, grid.cols / 2);
    dfs(&mut grid, start).0
}

pub fn part2(input: &str) -> u64 {
    let (line, input) = input.split_once('\n').unwrap();
    let mut top = vec![1; line.len()];
    input.lines().rev().for_each(|line| {
        line.bytes()
            .enumerate()
            .filter(|&(_, b)| b == b'^')
            .for_each(|(i, _)| top[i] = top[i - 1] + top[i + 1])
    });
    top[top.len() / 2]
}
