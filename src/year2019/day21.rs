use super::intcode;

const INSTRS: &str = "OR A T\n\
                      AND B T\n\
                      AND C T\n\
                      NOT T J\n\
                      AND D J\n\
                      WALK\n";

fn run(input: &str, instrs: &str) -> Option<i64> {
    let mut prog = intcode::new(input);
    for c in instrs.chars() {
        prog.input.push_back(c as i64);
    }
    prog.run();
    prog.output.drain(..).next_back()
}

pub fn part1(input: &str) -> Option<i64> {
    run(input, INSTRS)
}

const INSTRS2: &str = "OR A T\n\
                       AND B T\n\
                       AND C T\n\
                       NOT T J\n\
                       AND D J\n\
                       NOT J T\n\
                       OR E T\n\
                       OR H T\n\
                       AND T J\n\
                       RUN\n";

pub fn part2(input: &str) -> Option<i64> {
    run(input, INSTRS2)
}
