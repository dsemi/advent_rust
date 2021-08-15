use std::ops::Add;

#[derive(Clone, Copy)]
struct Coord3(i64, i64, i64);

impl Add for Coord3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Coord3(x1, y1, z1) = self;
        let Coord3(x2, y2, z2) = other;
        Coord3(x1 + x2, y1 + y2, z1 + z2)
    }
}

fn dist_from_origin(pos: Coord3) -> i64 {
    let Coord3(x, y, z) = pos;
    [x, y, z].iter().map(|v| v.abs()).max().unwrap()
}

fn ap(d: &str, p: Coord3) -> Coord3 {
    match d {
        "n" => Coord3(0, 1, -1) + p,
        "ne" => Coord3(1, 0, -1) + p,
        "se" => Coord3(1, -1, 0) + p,
        "s" => Coord3(0, -1, 1) + p,
        "sw" => Coord3(-1, 0, 1) + p,
        "nw" => Coord3(-1, 1, 0) + p,
        _ => panic!("Parse error: {}", d),
    }
}

fn path(input: &str) -> impl Iterator<Item = Coord3> + '_ {
    input.split(',').scan(Coord3(0, 0, 0), |acc, x| {
        *acc = ap(x, *acc);
        Some(*acc)
    })
}

pub fn part1(input: &str) -> Option<i64> {
    path(input).last().map(dist_from_origin)
}

pub fn part2(input: &str) -> Option<i64> {
    path(input).map(dist_from_origin).max()
}
