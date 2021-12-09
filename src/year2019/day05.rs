use super::intcode;

pub fn part1(input: &str) -> Option<i64> {
    intcode::new(input).run_with_input(&[1]).last()
}

pub fn part2(input: &str) -> Option<i64> {
    intcode::new(input).run_with_input(&[5]).last()
}
