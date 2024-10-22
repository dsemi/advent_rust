use crate::utils::*;

fn dist_from_origin(pos: C3<i64>) -> i64 {
    [pos.0, pos.1, pos.2].iter().map(|v| v.abs()).max().unwrap()
}

fn ap(p: &C3<i64>, d: &str) -> C3<i64> {
    let x = match d {
        "n" => C3(0, 1, -1),
        "ne" => C3(1, 0, -1),
        "se" => C3(1, -1, 0),
        "s" => C3(0, -1, 1),
        "sw" => C3(-1, 0, 1),
        "nw" => C3(-1, 1, 0),
        _ => panic!("Parse error: {}", d),
    };
    *p + x
}

fn path(input: &str) -> impl Iterator<Item = C3<i64>> + use<'_> {
    input.split(',').scan(C3(0, 0, 0), |acc, p| {
        *acc = ap(acc, p);
        Some(*acc)
    })
}

pub fn part1(input: &str) -> Option<i64> {
    path(input).last().map(dist_from_origin)
}

pub fn part2(input: &str) -> Option<i64> {
    path(input).map(dist_from_origin).max()
}
