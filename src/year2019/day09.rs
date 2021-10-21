use crate::year2019::intcode;

pub fn part1(input: &str) -> Option<i64> {
    intcode::new(input).run_with_input(&[1]).next()
}

pub fn part2(input: &str) -> Option<i64> {
    intcode::new(input).run_with_input(&[2]).next()
}
