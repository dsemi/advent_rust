use crate::utils::ocr::*;
use crate::utils::parsers::*;
use crate::utils::C;
use ahash::AHashSet;
use std::cmp::{max, min};

struct Obj {
    pos: C<i32>,
    vel: C<i32>,
}

fn object(i: &str) -> IResult<&str, Obj> {
    let (i, pos) = delimited(tag("position=<"), map(coord(i32), Into::into), tag("> "))(i)?;
    let (i, vel) = delimited(tag("velocity=<"), map(coord(i32), Into::into), tag(">"))(i)?;
    Ok((i, Obj { pos, vel }))
}

fn bounding_box(objs: &[Obj]) -> (i32, i32, i32, i32) {
    let mut result = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for obj in objs {
        result.0 = min(result.0, obj.pos.0);
        result.1 = min(result.1, obj.pos.1);
        result.2 = max(result.2, obj.pos.0);
        result.3 = max(result.3, obj.pos.1);
    }
    result
}

fn find_message(objs: &mut [Obj]) -> usize {
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
    let mut result = String::new();
    for y in y0..=y1 {
        result.push('\n');
        for x in x0..=x1 {
            result.push(if lights.contains(&C(x, y)) { '#' } else { ' ' });
        }
    }
    parse_letters(&result, None)
}

pub fn part1(input: &str) -> String {
    let mut objs = lines(object).read(input);
    find_message(&mut objs);
    show_objects(&objs)
}

pub fn part2(input: &str) -> usize {
    let mut objs = lines(object).read(input);
    find_message(&mut objs)
}
