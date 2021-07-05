use counter::Counter;
use std::collections::HashMap;

use crate::utils::Coord;

fn parse_landscape(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn neighbors<'a>(grid: &'a Vec<Vec<char>>, c: Coord<i32>) -> impl Iterator<Item = char> + 'a {
    vec![
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(0, -1),
        Coord::new(0, 1),
        Coord::new(-1, -1),
        Coord::new(-1, 1),
        Coord::new(1, -1),
        Coord::new(1, 1),
    ]
    .into_iter()
    .filter_map(move |d| {
        let c2 = c + d;
        (c2.x >= 0 && c2.x < grid.len() as i32 && c2.y >= 0 && c2.y < grid[c2.x as usize].len() as i32)
            .then(|| grid[c2.x as usize][c2.y as usize])
    })
}

fn step(grid: &mut Vec<Vec<char>>) {
    let mut counts = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        counts.push(Vec::new());
        for (c, _) in row.iter().enumerate() {
            counts[r].push(neighbors(grid, Coord::new(r as i32, c as i32)).collect::<Counter<_>>());
        }
    }
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            if *v == '.' && counts[r][c][&'|'] >= 3 {
                *v = '|';
            } else if *v == '|' && counts[r][c][&'#'] >= 3 {
                *v = '#';
            } else if *v == '#' && (counts[r][c][&'#'] < 1 || counts[r][c][&'|'] < 1) {
                *v = '.';
            }
        }
    }
}

fn resource_value(grid: &Vec<Vec<char>>) -> usize {
    let mut ws = 0;
    let mut ls = 0;
    for row in grid {
        for v in row {
            if *v == '|' {
                ws += 1;
            } else if *v == '#' {
                ls += 1;
            }
        }
    }
    ws * ls
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_landscape(input);
    for _ in 0..10 {
        step(&mut grid);
    }
    resource_value(&grid)
}

pub fn part2(input: &str) -> usize {
    let n: usize = 1_000_000_000;
    let mut t: HashMap<usize, (usize, Vec<Vec<char>>)> = HashMap::new();
    let mut grid = parse_landscape(input);
    for c in (0..n+1).rev() {
        let r = resource_value(&grid);
        match t.get(&r) {
            Some((n, g)) if g == &grid => {
                let steps_away = c.rem_euclid(n - c);
                for _ in 0..steps_away {
                    step(&mut grid);
                }
                return resource_value(&grid);
            },
            _ => {
                t.insert(r, (c, grid.clone()));
                step(&mut grid);
            },
        }
    }
    panic!("No solution found")
}
