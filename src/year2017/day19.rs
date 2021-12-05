use crate::utils::*;
use ahash::AHashMap;
use lazy_static::lazy_static;

fn parse_grid(input: &str) -> AHashMap<Coord<i32>, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(c, v)| (v != ' ').then(|| (Coord::new(r as i32, c as i32), v)))
        })
        .collect()
}

fn turn(grid: &AHashMap<Coord<i32>, char>, dir: Coord<i32>, pos: Coord<i32>) -> Coord<i32> {
    lazy_static! {
        static ref LEFT: Coord<i32> = Coord::new(0, 1);
        static ref RIGHT: Coord<i32> = Coord::new(0, -1);
    }

    if grid.contains_key(&(*LEFT * dir + pos)) {
        *LEFT * dir
    } else {
        *RIGHT * dir
    }
}

fn follow_path(grid: AHashMap<Coord<i32>, char>) -> Vec<char> {
    let mut coord = *grid.keys().min().unwrap();
    let mut dir = Coord::new(1, 0);
    let mut result = Vec::new();
    while grid.contains_key(&coord) {
        let v = grid[&coord];
        result.push(v);
        if v == '+' {
            dir = turn(&grid, dir, coord);
        }
        coord += dir;
    }
    result
}

pub fn part1(input: &str) -> String {
    follow_path(parse_grid(input))
        .into_iter()
        .filter(|x| !"|-+".contains(*x))
        .collect()
}

pub fn part2(input: &str) -> usize {
    follow_path(parse_grid(input)).len()
}
