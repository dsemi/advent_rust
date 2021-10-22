use crate::utils::*;

fn dist_from_origin(pos: Coord3<i64>) -> i64 {
    [pos.x, pos.y, pos.z].iter().map(|v| v.abs()).max().unwrap()
}

fn ap(p: &Coord3<i64>, d: &str) -> Coord3<i64> {
    let x = match d {
        "n" => Coord3::new(0, 1, -1),
        "ne" => Coord3::new(1, 0, -1),
        "se" => Coord3::new(1, -1, 0),
        "s" => Coord3::new(0, -1, 1),
        "sw" => Coord3::new(-1, 0, 1),
        "nw" => Coord3::new(-1, 1, 0),
        _ => panic!("Parse error: {}", d),
    };
    *p + x
}

fn path(input: &str) -> impl Iterator<Item = Coord3<i64>> + '_ {
    input.split(',').good_scan(Coord3::new(0, 0, 0), ap)
}

pub fn part1(input: &str) -> Option<i64> {
    path(input).last().map(dist_from_origin)
}

pub fn part2(input: &str) -> Option<i64> {
    path(input).map(dist_from_origin).max()
}
