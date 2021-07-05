use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    intcode::run_with_input(vec![1], intcode::new(input))[0]
}

pub fn part2(input: &str) -> i64 {
    intcode::run_with_input(vec![2], intcode::new(input))[0]
}
