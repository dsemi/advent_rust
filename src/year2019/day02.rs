use crate::year2019::intcode;

pub fn part1(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    intcode::run_no_io(12, 2, mem)
}

pub fn part2(input: &str) -> i64 {
    let mem = intcode::parse_instrs(input);
    for noun in 0..99 {
        for verb in 0..99 {
            if intcode::run_no_io(noun, verb, mem.clone()) == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    0
}
