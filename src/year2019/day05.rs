use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    *intcode::run_with_input(vec![1], mem).last().unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    *intcode::run_with_input(vec![5], mem).last().unwrap()
}
