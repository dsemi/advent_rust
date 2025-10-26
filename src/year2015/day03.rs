use crate::utils::*;
use hashbrown::HashSet;

fn unit_dir(c: char) -> C<i64> {
    match c {
        '<' => C(-1, 0),
        '>' => C(1, 0),
        'v' => C(0, -1),
        '^' => C(0, 1),
        _ => panic!("Unknown direction"),
    }
}

fn locations(inp: impl Iterator<Item = char>) -> HashSet<C<i64>> {
    let mut s: HashSet<C<i64>> = [C(0, 0)].into();
    s.extend(inp.scan(C(0, 0), |loc, c| {
        *loc += unit_dir(c);
        Some(*loc)
    }));
    s
}

pub fn part1(input: &str) -> usize {
    locations(input.chars()).len()
}

pub fn part2(input: &str) -> usize {
    locations(input.chars().step_by(2)).union(&locations(input.chars().skip(1).step_by(2))).count()
}
