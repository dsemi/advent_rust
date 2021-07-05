use std::cmp::{max, min};
use std::collections::HashMap;

use crate::utils::Coord;
use crate::year2019::intcode;

fn run_robot(mut prog: intcode::Program, t: &mut HashMap<Coord<i64>, i64>) {
    let mut pos = Coord::new(0, 0);
    let mut dir = Coord::new(0, -1);
    while !prog.done {
        prog.input.push_back(*t.get(&pos).unwrap_or(&0));
        prog.run();
        match prog.output.drain(..).collect::<Vec<_>>()[..] {
            [col, d] => {
                t.insert(pos, col);
                dir *= if d == 1 {
                    Coord::new(0, 1)
                } else {
                    Coord::new(0, -1)
                };
                pos += dir;
            }
            _ => panic!("Invalid response"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut m = HashMap::new();
    run_robot(intcode::new(input), &mut m);
    m.len()
}

fn draw(points: &HashMap<Coord<i64>, i64>) -> String {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for pt in points.keys() {
        min_x = min(min_x, pt.x);
        min_y = min(min_y, pt.y);
        max_x = max(max_x, pt.x);
        max_y = max(max_y, pt.y);
    }
    let mut chrs = vec!['\n'];
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            chrs.push(if points.get(&Coord::new(x, y)).unwrap_or(&0) == &0 {
                ' '
            } else {
                '#'
            });
        }
        chrs.push('\n');
    }
    chrs.into_iter().collect()
}

pub fn part2(input: &str) -> String {
    let mut m = HashMap::new();
    m.insert(Coord::new(0, 0), 1);
    run_robot(intcode::new(input), &mut m);
    draw(&m)
}
