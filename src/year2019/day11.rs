use super::intcode;
use crate::utils::ocr::*;
use crate::utils::C;
use ahash::AHashMap;
use std::cmp::{max, min};

fn run_robot(mut prog: intcode::Program, t: &mut AHashMap<C<i64>, i64>) {
    let mut pos = C(0, 0);
    let mut dir = C(0, -1);
    while !prog.done {
        prog.input.push_back(*t.get(&pos).unwrap_or(&0));
        prog.run();
        let col = prog.output.pop_front().unwrap();
        t.insert(pos, col);
        let d = prog.output.pop_front().unwrap();
        dir *= if d == 1 { C(0, 1) } else { C(0, -1) };
        pos += dir;
    }
}

pub fn part1(input: &str) -> usize {
    let mut m = AHashMap::new();
    run_robot(intcode::new(input), &mut m);
    m.len()
}

fn draw(points: &AHashMap<C<i64>, i64>) -> String {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for &C(x, y) in points.keys() {
        min_x = min(min_x, x);
        min_y = min(min_y, y);
        max_x = max(max_x, x);
        max_y = max(max_y, y);
    }
    let mut chrs = vec![];
    for y in min_y..=max_y {
        chrs.push('\n');
        for x in min_x..=max_x {
            chrs.push(if points.get(&C(x, y)).unwrap_or(&0) == &0 {
                ' '
            } else {
                '#'
            });
        }
    }
    parse_letters(&chrs.into_iter().collect::<String>(), None)
}

pub fn part2(input: &str) -> String {
    let mut m = AHashMap::new();
    m.insert(C(0, 0), 1);
    run_robot(intcode::new(input), &mut m);
    draw(&m)
}
