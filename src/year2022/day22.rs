use crate::utils::C;
use itertools::iterate;
use regex::Regex;

fn valid(grid: &[Vec<char>], pos: C<i32>) -> Option<C<i32>> {
    pos.index_of(grid).filter(|&p| grid[p] != ' ')
}

fn walk<F>(input: &str, wrap: F) -> i32
where
    F: Fn(&[Vec<char>], C<i32>, C<i32>) -> (C<i32>, C<i32>),
{
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
                    let (pos2, dir2) = valid(&grid, pos + dir)
                        .map(|p| (p, dir))
                        .unwrap_or_else(|| wrap(&grid, pos, dir));
                    if grid[pos2] == '#' {
                        break;
                    }
                    (pos, dir) = (pos2, dir2);
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
    walk(input, |grid, C(r, c), C(dr, dc)| {
        let mr = grid.len() as i32;
        let mc = grid[r as usize].len() as i32;
        let pts: Box<dyn Iterator<Item = C<i32>>> = if dr != 0 {
            Box::new(iterate(r, |r| (r + dr).rem_euclid(mr)).map(|r| C(r, c)))
        } else {
            Box::new(iterate(c, |c| (c + dc).rem_euclid(mc)).map(|c| C(r, c)))
        };
        let pos = pts.skip(1).find(|&p| valid(grid, p).is_some()).unwrap();
        (pos, C(dr, dc))
    })
}

pub fn part2(input: &str) -> i32 {
    walk(input, |_, pos, dir| match (pos, dir) {
        (C(0, c), C(-1, 0)) if 50 <= c && c < 100 => (C(c + 100, 0), C(0, 1)),
        (C(r, 0), C(0, -1)) if 150 <= r && r < 200 => (C(0, r - 100), C(1, 0)),
        (C(0, c), C(-1, 0)) if 100 <= c && c < 150 => (C(199, c - 100), C(-1, 0)),
        (C(199, c), C(1, 0)) if 0 <= c && c < 50 => (C(0, c + 100), C(1, 0)),
        (C(r, 50), C(0, -1)) if 0 <= r && r < 50 => (C(149 - r, 0), C(0, 1)),
        (C(r, 0), C(0, -1)) if 100 <= r && r < 150 => (C(149 - r, 50), C(0, 1)),
        (C(r, 149), C(0, 1)) if 0 <= r && r < 50 => (C(149 - r, 99), C(0, -1)),
        (C(r, 99), C(0, 1)) if 100 <= r && r < 150 => (C(149 - r, 149), C(0, -1)),
        (C(49, c), C(1, 0)) if 100 <= c && c < 150 => (C(c - 50, 99), C(0, -1)),
        (C(r, 99), C(0, 1)) if 50 <= r && r < 100 => (C(49, r + 50), C(-1, 0)),
        (C(r, 50), C(0, -1)) if 50 <= r && r < 100 => (C(100, r - 50), C(1, 0)),
        (C(100, c), C(-1, 0)) if 0 <= c && c < 50 => (C(c + 50, 50), C(0, 1)),
        (C(149, c), C(1, 0)) if 50 <= c && c < 100 => (C(c + 100, 49), C(0, -1)),
        (C(r, 49), C(0, 1)) if 150 <= r && r < 200 => (C(149, r - 100), C(-1, 0)),
        _ => unreachable!(),
    })
}
