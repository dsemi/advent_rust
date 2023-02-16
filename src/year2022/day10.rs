use crate::ocr::*;
use itertools::Itertools;

fn run(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split_whitespace()
        .map(|tok| tok.parse().unwrap_or(0))
        .scan(1, |x, d| Some(std::mem::replace(x, *x + d)))
}

pub fn part1(input: &str) -> i32 {
    run(input)
        .zip(1..)
        .filter_map(|(x, c)| (c % 40 == 20).then(|| c * x))
        .sum()
}

pub fn part2(input: &str) -> String {
    let res = run(input)
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|c| c.iter().zip(0..))
        .map(|c| c.map(|(x, m)| if (m - x).abs() <= 1 { '#' } else { ' ' }))
        .map(|c| c.collect::<String>())
        .join("\n");
    parse_letters(&res, None)
}
