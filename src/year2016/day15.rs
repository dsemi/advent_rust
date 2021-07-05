use regex::Regex;

use crate::utils::*;

fn parse_discs(input: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            match re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect::<Vec<i64>>()[..]
            {
                [disc_num, modulo, _, pos] => (-pos - disc_num, modulo),
                _ => panic!("Parse failure: {}", line),
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    chinese_remainder(parse_discs(input))
}

pub fn part2(input: &str) -> i64{
    chinese_remainder(parse_discs(&format!(
        "{}\n{}",
        input, "Disc #7 has 11 positions; at time=0, it is at position 0."
    )))
}
