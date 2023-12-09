use crate::utils::parsers::*;
use crate::utils::*;

fn parse_discs(input: &str) -> Vec<(i64, i64)> {
    lines_iter(
        input,
        (
            delimited("Disc #", i64, " has "),
            terminated(i64, " positions; at time=0, it is at position "),
            terminated(i64, '.'),
        ),
    )
    .map(|(disc_num, modulo, pos)| (-pos - disc_num, modulo))
    .collect()
}

pub fn part1(input: &str) -> i64 {
    chinese_remainder(parse_discs(input))
}

pub fn part2(input: &str) -> i64 {
    chinese_remainder(parse_discs(&format!(
        "{input}\nDisc #7 has 11 positions; at time=0, it is at position 0."
    )))
}
