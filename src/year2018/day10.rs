use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;
use std::cmp::{max, min};

use crate::utils::Coord;

struct Obj {
    pos: Coord<i32>,
    vel: Coord<i32>,
}

fn parse_objects(input: &str) -> Vec<Obj> {
    input
        .lines()
        .map(|line| {
            let (x0, y0, x1, y1) = scanf!(
                line,
                "position=<{},{}> velocity=<{},{}>",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            Obj {
                pos: Coord::new(x0, y0),
                vel: Coord::new(x1, y1),
            }
        })
        .collect()
}

fn bounding_box(objs: &[Obj]) -> (i32, i32, i32, i32) {
    let mut result = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for obj in objs {
        result.0 = min(result.0, obj.pos.x);
        result.1 = min(result.1, obj.pos.y);
        result.2 = max(result.2, obj.pos.x);
        result.3 = max(result.3, obj.pos.y);
    }
    result
}

fn find_message(objs: &mut Vec<Obj>) -> usize {
    let mut bb = bounding_box(objs);
    let mut result = 0;
    while bb.3 - bb.1 > 15 {
        for obj in objs.iter_mut() {
            obj.pos += obj.vel;
        }
        bb = bounding_box(objs);
        result += 1;
    }
    result
}

fn show_objects(objs: &[Obj]) -> String {
    let lights = objs.iter().map(|obj| obj.pos).collect::<AHashSet<_>>();
    let (x0, y0, x1, y1) = bounding_box(objs);
    let mut result = "\n".to_owned();
    for y in y0..=y1 {
        for x in x0..=x1 {
            result.push(if lights.contains(&Coord::new(x, y)) {
                '#'
            } else {
                ' '
            });
        }
        result.push('\n');
    }
    result
}

pub fn part1(input: &str) -> String {
    let mut objs = parse_objects(input);
    find_message(&mut objs);
    show_objects(&objs)
}

pub fn part2(input: &str) -> usize {
    let mut objs = parse_objects(input);
    find_message(&mut objs)
}
