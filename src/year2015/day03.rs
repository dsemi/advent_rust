use num_complex::Complex;
use std::collections::HashSet;

use crate::utils::*;

fn locations(inp: impl Iterator<Item = char>) -> HashSet<Complex<i64>> {
    inp.scanl(Complex::new(0, 0), |loc, c| loc + unit_dir(c))
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
