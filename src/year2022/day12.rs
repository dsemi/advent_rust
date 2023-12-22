use crate::utils::*;

fn neighbors(grid: &Grid<u8>, pos: &C<i32>) -> Vec<C<i32>> {
    let curr_h = grid[*pos];
    [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)]
        .into_iter()
        .filter_map(|c| {
            let pos2 = pos + c;
            grid.get(pos2).filter(|&v| *v <= curr_h + 1).map(|_| pos2)
        })
        .collect()
}

fn solve(input: &str, sts: &[u8]) -> Option<usize> {
    let mut starts = Vec::new();
    let mut done = C(0, 0);
    let mut grid: Grid<u8> = input.bytes().collect();
    for (C(r, c), v) in grid.idx_iter_mut() {
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
    bfs_m(starts, move |p| neighbors(&grid, p)).find_map(|(d, p)| (p == done).then_some(d))
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, &[b'S'])
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, &[b'S', b'a'])
}
