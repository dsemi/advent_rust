use crate::utils::Coord;
use ahash::AHashMap;
use regex::Regex;

type Portal = (Coord<i32>, Coord<i32>);

fn walk(input: &str, portals: AHashMap<Portal, Portal>) -> i32 {
    let pts = input.split("\n\n").collect::<Vec<_>>();
    let grid: Vec<Vec<char>> = pts[0].lines().map(|line| line.chars().collect()).collect();
    let mut pos = Coord::new(0, grid[0].iter().position(|&c| c == '.').unwrap() as i32);
    let mut dir = Coord::new(0, 1);
    let reg = Regex::new(r"\d+|.").unwrap();
    for instr in reg.find_iter(pts[1]) {
        match instr.as_str() {
            "L" => dir *= Coord::new(0, 1),
            "R" => dir *= Coord::new(0, -1),
            n => {
                for _ in 0..n.parse().unwrap() {
                    let (pos2, dir2) = if let Some((p, d)) = portals.get(&(pos, dir)) {
                        (*p, *d)
                    } else {
                        (pos + dir, dir)
                    };
                    if grid[pos2] == '#' {
                        break;
                    }
                    pos = pos2;
                    dir = dir2;
                }
            }
        }
    }
    let row = pos.x + 1;
    let col = pos.y + 1;
    let facing = match (dir.x, dir.y) {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * col + facing
}

pub fn part1(input: &str) -> i32 {
    let mut portals = AHashMap::new();
    for i in 0..50 {
        portals.insert(
            (Coord::new(0, 50 + i), Coord::new(-1, 0)),
            (Coord::new(149, 50 + i), Coord::new(-1, 0)),
        );
        portals.insert(
            (Coord::new(149, 50 + i), Coord::new(1, 0)),
            (Coord::new(0, 50 + i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(0, 100 + i), Coord::new(-1, 0)),
            (Coord::new(49, 100 + i), Coord::new(-1, 0)),
        );
        portals.insert(
            (Coord::new(49, 100 + i), Coord::new(1, 0)),
            (Coord::new(0, 100 + i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(i, 50), Coord::new(0, -1)),
            (Coord::new(i, 149), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(i, 149), Coord::new(0, 1)),
            (Coord::new(i, 50), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(50 + i, 50), Coord::new(0, -1)),
            (Coord::new(50 + i, 99), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(50 + i, 99), Coord::new(0, 1)),
            (Coord::new(50 + i, 50), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(100, i), Coord::new(-1, 0)),
            (Coord::new(199, i), Coord::new(-1, 0)),
        );
        portals.insert(
            (Coord::new(199, i), Coord::new(1, 0)),
            (Coord::new(100, i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(100 + i, 0), Coord::new(0, -1)),
            (Coord::new(100 + i, 99), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(100 + i, 99), Coord::new(0, 1)),
            (Coord::new(100 + i, 0), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(150 + i, 0), Coord::new(0, -1)),
            (Coord::new(150 + i, 49), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(150 + i, 49), Coord::new(0, 1)),
            (Coord::new(150 + i, 0), Coord::new(0, 1)),
        );
    }
    walk(input, portals)
}

pub fn part2(input: &str) -> i32 {
    let mut portals = AHashMap::new();
    for i in 0..50 {
        portals.insert(
            (Coord::new(0, 50 + i), Coord::new(-1, 0)),
            (Coord::new(150 + i, 0), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(150 + i, 0), Coord::new(0, -1)),
            (Coord::new(0, 50 + i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(0, 100 + i), Coord::new(-1, 0)),
            (Coord::new(199, i), Coord::new(-1, 0)),
        );
        portals.insert(
            (Coord::new(199, i), Coord::new(1, 0)),
            (Coord::new(0, 100 + i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(i, 50), Coord::new(0, -1)),
            (Coord::new(149 - i, 0), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(149 - i, 0), Coord::new(0, -1)),
            (Coord::new(i, 50), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(i, 149), Coord::new(0, 1)),
            (Coord::new(149 - i, 99), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(149 - i, 99), Coord::new(0, 1)),
            (Coord::new(i, 149), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(49, 100 + i), Coord::new(1, 0)),
            (Coord::new(50 + i, 99), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(50 + i, 99), Coord::new(0, 1)),
            (Coord::new(49, 100 + i), Coord::new(-1, 0)),
        );
        portals.insert(
            (Coord::new(50 + i, 50), Coord::new(0, -1)),
            (Coord::new(100, i), Coord::new(1, 0)),
        );
        portals.insert(
            (Coord::new(100, i), Coord::new(-1, 0)),
            (Coord::new(50 + i, 50), Coord::new(0, 1)),
        );
        portals.insert(
            (Coord::new(149, 50 + i), Coord::new(1, 0)),
            (Coord::new(150 + i, 49), Coord::new(0, -1)),
        );
        portals.insert(
            (Coord::new(150 + i, 49), Coord::new(0, 1)),
            (Coord::new(149, 50 + i), Coord::new(-1, 0)),
        );
    }
    walk(input, portals)
}
