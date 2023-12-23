use crate::utils::parsers::*;
use crate::utils::*;
use std::cmp::max;

fn solve(input: &str, p2: bool) -> usize {
    let mut lines = vec![];
    let (mut max_x, mut max_y) = (0, 0);
    for line in input.lines() {
        let ((x0, y0), (x1, y1)) = sep2(coord(i32), "->").read(line);
        max_x = max(max_x, max(x0, x1));
        max_y = max(max_y, max(y0, y1));
        lines.push((C(x0, y0), C(x1, y1)));
    }
    let mut grid: Grid<usize, i32> = Grid::new(max_x + 1, max_y + 1);
    for (mut c, c1) in lines {
        if !p2 && c.0 != c1.0 && c.1 != c1.1 {
            continue;
        }
        let d = (c1 - c).signum();
        while c != c1 + d {
            grid[c] += 1;
            c += d;
        }
    }
    grid.into_iter().filter(|&v| v > 1).count()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
