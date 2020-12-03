use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    intcode::run_with_input(vec![1], mem)[0]
}

pub fn part2(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    intcode::run_with_input(vec![2], mem)[0]
}
