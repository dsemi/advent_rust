use crate::utils::*;

fn neighbors(grid: &Vec<Vec<u8>>, pos: &Coord<i32>) -> Vec<Coord<i32>> {
    let curr_h = grid[*pos];
    [
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(0, -1),
        Coord::new(0, 1),
    ]
    .into_iter()
    .filter_map(|c| {
        let pos2 = *pos + c;
        (pos2.x >= 0
            && pos2.x < grid.len() as i32
            && pos2.y >= 0
            && pos2.y < grid[0].len() as i32
            && grid[pos2] <= curr_h + 1)
            .then(|| pos2)
    })
    .collect()
}

fn solve(input: &str, sts: &[u8]) -> usize {
    let mut starts = Vec::new();
    let mut done = Coord::new(0, 0);
    let mut grid: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            if sts.contains(v) {
                starts.push(Coord::new(r as i32, c as i32));
            }
            if v == &b'S' {
                *v = b'a';
            } else if v == &b'E' {
                done = Coord::new(r as i32, c as i32);
                *v = b'z';
            }
        }
    }
    for (d, p) in bfs_m(starts, |p| neighbors(&grid, p)) {
        if p == done {
            return d;
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    solve(input, &[b'S'])
}

pub fn part2(input: &str) -> usize {
    solve(input, &[b'S', b'a'])
}
