use crate::utils::*;
use scan_fmt::scan_fmt as scanf;

fn parse_discs(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (disc_num, modulo, pos) = scanf!(
                line,
                "Disc #{} has {} positions; at time=0, it is at position {}.",
                i64,
                i64,
                i64
            )
            .unwrap();
            (-pos - disc_num, modulo)
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    chinese_remainder(parse_discs(input))
}

pub fn part2(input: &str) -> i64 {
    chinese_remainder(parse_discs(&format!(
        "{}\n{}",
        input, "Disc #7 has 11 positions; at time=0, it is at position 0."
    )))
}
