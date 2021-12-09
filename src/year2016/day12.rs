use super::assembunny;

pub fn part1(input: &str) -> i64 {
    let mut sim = assembunny::parse_instrs(input);
    sim.run();
    sim.regs[0]
}

pub fn part2(input: &str) -> i64 {
    let mut sim = assembunny::parse_instrs(input);
    sim.regs[2] = 1;
    sim.run();
    sim.regs[0]
}
