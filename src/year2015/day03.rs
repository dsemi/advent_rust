use crate::utils::*;
use ahash::AHashSet;

fn unit_dir(c: char) -> C<i64> {
    match c {
        '<' => C(-1, 0),
        '>' => C(1, 0),
        'v' => C(0, -1),
        '^' => C(0, 1),
        _ => panic!("Unknown direction"),
    }
}

fn locations(inp: impl Iterator<Item = char>) -> AHashSet<C<i64>> {
    inp.good_scan(C(0, 0), |loc, c| *loc + unit_dir(c))
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
