use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    *intcode::run_with_input(vec![1], intcode::new(input)).last().unwrap()
}

pub fn part2(input: &str) -> i64 {
    *intcode::run_with_input(vec![5], intcode::new(input)).last().unwrap()
}
