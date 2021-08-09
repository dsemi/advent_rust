use ahash::AHashSet;

use crate::utils::*;

fn locations(inp: impl Iterator<Item = char>) -> AHashSet<Coord<i64>> {
    std::iter::once(Coord::new(0, 0))
        .chain(inp.scan(Coord::new(0, 0), |loc, c| {
            *loc += unit_dir(c);
            Some(*loc)
        }))
        .collect()
}

pub fn part1(input: &str) -> usize {
    locations(input.chars()).len()
}

pub fn part2(input: &str) -> usize {
    locations(input.chars().step_by(2))
        .union(&locations(input.chars().skip(1).step_by(2)))
        .count()
}
