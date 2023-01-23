use crate::utils::*;

fn neighbors(grid: &Vec<Vec<u8>>, pos: &C<i32>) -> Vec<C<i32>> {
    let curr_h = grid[*pos];
    [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)]
        .into_iter()
        .filter_map(|c| {
            let pos2 = *pos + c;
            (pos2.0 >= 0
                && pos2.0 < grid.len() as i32
                && pos2.1 >= 0
                && pos2.1 < grid[0].len() as i32
                && grid[pos2] <= curr_h + 1)
                .then(|| pos2)
        })
        .collect()
}

fn solve(input: &str, sts: &[u8]) -> Option<usize> {
    let mut starts = Vec::new();
    let mut done = C(0, 0);
    let mut grid: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            if sts.contains(v) {
                starts.push(C(r as i32, c as i32));
            }
            if v == &b'S' {
                *v = b'a';
            } else if v == &b'E' {
                done = C(r as i32, c as i32);
                *v = b'z';
            }
        }
    }
    bfs_m(starts, move |p| neighbors(&grid, p)).find_map(|(d, p)| (p == done).then(|| d))
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, &[b'S'])
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, &[b'S', b'a'])
}
