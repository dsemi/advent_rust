use crate::utils::C;
use ahash::AHashMap;
use regex::Regex;

type Portal = (C<i32>, C<i32>);

fn walk(input: &str, portals: AHashMap<Portal, Portal>) -> i32 {
    let pts = input.split("\n\n").collect::<Vec<_>>();
    let grid: Vec<Vec<char>> = pts[0].lines().map(|line| line.chars().collect()).collect();
    let mut pos = C(0, grid[0].iter().position(|&c| c == '.').unwrap() as i32);
    let mut dir = C(0, 1);
    let reg = Regex::new(r"\d+|.").unwrap();
    for instr in reg.find_iter(pts[1]) {
        match instr.as_str() {
            "L" => dir *= C(0, 1),
            "R" => dir *= C(0, -1),
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
    let C(row, col) = pos + C(1, 1);
    let facing = match dir {
        C(0, 1) => 0,
        C(1, 0) => 1,
        C(0, -1) => 2,
        C(-1, 0) => 3,
        _ => unreachable!(),
    };
    1000 * row + 4 * col + facing
}

pub fn part1(input: &str) -> i32 {
    let mut portals = AHashMap::new();
    for i in 0..50 {
        for (k, v) in [
            ((C(0, 50 + i), C(-1, 0)), (C(149, 50 + i), C(-1, 0))),
            ((C(149, 50 + i), C(1, 0)), (C(0, 50 + i), C(1, 0))),
            ((C(0, 100 + i), C(-1, 0)), (C(49, 100 + i), C(-1, 0))),
            ((C(49, 100 + i), C(1, 0)), (C(0, 100 + i), C(1, 0))),
            ((C(i, 50), C(0, -1)), (C(i, 149), C(0, -1))),
            ((C(i, 149), C(0, 1)), (C(i, 50), C(0, 1))),
            ((C(50 + i, 50), C(0, -1)), (C(50 + i, 99), C(0, -1))),
            ((C(50 + i, 99), C(0, 1)), (C(50 + i, 50), C(0, 1))),
            ((C(100, i), C(-1, 0)), (C(199, i), C(-1, 0))),
            ((C(199, i), C(1, 0)), (C(100, i), C(1, 0))),
            ((C(100 + i, 0), C(0, -1)), (C(100 + i, 99), C(0, -1))),
            ((C(100 + i, 99), C(0, 1)), (C(100 + i, 0), C(0, 1))),
            ((C(150 + i, 0), C(0, -1)), (C(150 + i, 49), C(0, -1))),
            ((C(150 + i, 49), C(0, 1)), (C(150 + i, 0), C(0, 1))),
        ] {
            portals.insert(k, v);
        }
    }
    walk(input, portals)
}

pub fn part2(input: &str) -> i32 {
    let mut portals = AHashMap::new();
    for i in 0..50 {
        for (k, v) in [
            ((C(0, 50 + i), C(-1, 0)), (C(150 + i, 0), C(0, 1))),
            ((C(150 + i, 0), C(0, -1)), (C(0, 50 + i), C(1, 0))),
            ((C(0, 100 + i), C(-1, 0)), (C(199, i), C(-1, 0))),
            ((C(199, i), C(1, 0)), (C(0, 100 + i), C(1, 0))),
            ((C(i, 50), C(0, -1)), (C(149 - i, 0), C(0, 1))),
            ((C(149 - i, 0), C(0, -1)), (C(i, 50), C(0, 1))),
            ((C(i, 149), C(0, 1)), (C(149 - i, 99), C(0, -1))),
            ((C(149 - i, 99), C(0, 1)), (C(i, 149), C(0, -1))),
            ((C(49, 100 + i), C(1, 0)), (C(50 + i, 99), C(0, -1))),
            ((C(50 + i, 99), C(0, 1)), (C(49, 100 + i), C(-1, 0))),
            ((C(50 + i, 50), C(0, -1)), (C(100, i), C(1, 0))),
            ((C(100, i), C(-1, 0)), (C(50 + i, 50), C(0, 1))),
            ((C(149, 50 + i), C(1, 0)), (C(150 + i, 49), C(0, -1))),
            ((C(150 + i, 49), C(0, 1)), (C(149, 50 + i), C(-1, 0))),
        ] {
            portals.insert(k, v);
        }
    }
    walk(input, portals)
}
