use crate::utils::parsers2::*;
use std::cmp::{max, min};

const WIDTH: usize = 1000;

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for line in input.lines() {
        let pts: Vec<_> = separated(1.., coord(usize), "->").read(line);
        res.resize(
            max(res.len(), pts.iter().map(|(_, r)| r).max().unwrap() + 2),
            vec!['-'; WIDTH],
        );
        for i in 1..pts.len() {
            let (x0, y0) = min(pts[i - 1], pts[i]);
            let (x1, y1) = max(pts[i - 1], pts[i]);
            for x in x0..=x1 {
                for col in res.iter_mut().take(y1 + 1).skip(y0) {
                    col[x] = '#';
                }
            }
        }
    }
    res
}

fn go(grid: &mut Vec<Vec<char>>, p2: bool, coord: (usize, usize)) -> bool {
    let (r, c) = coord;
    if r >= grid.len() {
        return p2;
    }
    let v = grid[r][c];
    if v == '~' {
        return false;
    } else if v == '#' || v == 'o' {
        return true;
    }
    let b =
        go(grid, p2, (r + 1, c)) && go(grid, p2, (r + 1, c - 1)) && go(grid, p2, (r + 1, c + 1));
    grid[r][c] = if b { 'o' } else { '~' };
    b
}

fn flow_sand(mut grid: Vec<Vec<char>>, p2: bool) -> usize {
    go(&mut grid, p2, (0, 500));
    grid.into_iter()
        .map(|row| row.into_iter().filter(|&v| v == 'o').count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    flow_sand(parse_grid(input), false)
}

pub fn part2(input: &str) -> usize {
    flow_sand(parse_grid(input), true)
}
