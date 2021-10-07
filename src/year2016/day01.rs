use ahash::AHashSet;
use std::iter;

use crate::utils::Coord;

fn path(input: &str) -> impl Iterator<Item = Coord<i32>> + '_ {
    input
        .split(", ")
        .flat_map(|x| {
            let df = match x.chars().next().unwrap() {
                'R' => Coord::new(0, -1),
                'L' => Coord::new(0, 1),
                _ => panic!("Invalid dir {}", x),
            };
            let n: usize = x[1..].parse().unwrap();
            iter::once(df).chain(iter::repeat(Coord::new(1, 0)).take(n - 1))
        })
        .scan((Coord::new(0, 0), Coord::new(0, 1)), |state, x| {
            (*state).1 *= x;
            (*state).0 += state.1;
            Some(state.0)
        })
}

pub fn part1(input: &str) -> i32 {
    let pos = path(input).last().unwrap();
    pos.x.abs() + pos.y.abs()
}

pub fn part2(input: &str) -> i32 {
    let mut s = AHashSet::new();
    for pos in path(input) {
        if s.contains(&pos) {
            return pos.x.abs() + pos.y.abs();
        }
        s.insert(pos);
    }
    -1
}
