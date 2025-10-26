use crate::utils::parsers::*;
use Instr::*;

#[derive(Clone)]
enum Instr {
    Enable,
    Disable,
    Mul(u32, u32),
    Ignore,
}

fn memory(i: &mut &str) -> Result<Instr> {
    alt((
        "do()".value(Enable),
        "don't()".value(Disable),
        delimited("mul(", sep2(u32, ','), ')').map(|(a, b)| Mul(a, b)),
        any.value(Ignore),
    ))
    .parse_next(i)
}

pub fn part1(input: &str) -> u32 {
    iterator(input, memory).map(|m| if let Mul(a, b) = m { a * b } else { 0 }).sum()
}

pub fn part2(input: &str) -> u32 {
    iterator(input, memory)
        .fold((0, true), |(sum, enable), m| match m {
            Enable => (sum, true),
            Disable => (sum, false),
            Mul(a, b) if enable => (sum + a * b, enable),
            _ => (sum, enable),
        })
        .0
}
