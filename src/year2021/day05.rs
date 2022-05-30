use crate::utils::*;
use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

fn solve(input: &str, p2: bool) -> usize {
    let mut lines = vec![];
    let (mut max_x, mut max_y) = (0, 0);
    for line in input.lines() {
        let (x0, y0, x1, y1) = scanf!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
        max_x = max(max_x, max(x0, x1));
        max_y = max(max_y, max(y0, y1));
        lines.push((Coord::new(x0, y0), Coord::new(x1, y1)));
    }
    let mut grid = vec![vec![0; max_y as usize + 1]; max_x as usize + 1];
    for (mut c, c1) in lines {
        if !p2 && c.x != c1.x && c.y != c1.y {
            continue;
        }
        let d = (c1 - c).signum();
        while c != c1 + d {
            grid[c] += 1;
            c += d;
        }
    }
    grid.into_iter().flatten().filter(|&v| v > 1).count()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
