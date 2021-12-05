use crate::utils::Coord;
use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

fn parse(input: &str) -> Vec<(Coord<i32>, Coord<i32>)> {
    input
        .lines()
        .map(|line| {
            let (x0, y0, x1, y1) = scanf!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
            (Coord::new(x0, y0), Coord::new(x1, y1))
        })
        .collect()
}

fn solve(input: &str, p2: bool) -> usize {
    let lines = parse(input);
    let mut grid =
        vec![
            vec![0; lines.iter().map(|(c0, c1)| max(c0.y, c1.y)).max().unwrap() as usize + 1];
            lines.iter().map(|(c0, c1)| max(c0.x, c1.x)).max().unwrap() as usize + 1
        ];
    for (mut c0, c1) in lines {
        if !p2 && c0.x != c1.x && c0.y != c1.y {
            continue;
        }
        let d = (c1 - c0).signum();
        while c0 != c1 + d {
            grid[c0.x as usize][c0.y as usize] += 1;
            c0 += d;
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
