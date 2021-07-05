use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    intcode::run_no_io(12, 2, intcode::new(input))
}

pub fn part2(input: &str) -> i64 {
    let prog = intcode::new(input);
    for noun in 0..99 {
        for verb in 0..99 {
            if intcode::run_no_io(noun, verb, prog.clone()) == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    0
}
