use crate::utils::parsers::*;
use crate::utils::*;
use num_traits::Pow;

pub fn part1(input: &str) -> i64 {
    let input = input.replace(|c: char| !c.is_ascii_digit(), " ");
    let (r, c) = delimited(space0, sep2(i64, space1), space0).read(input.as_str());
    let n = r + c - 1;
    let index = n * (n - 1) / 2 + c - 1;
    (Mod::<33554393>(252533).pow(index) * Mod(20151125)).0
}

pub fn part2(_input: &str) -> &str {
    " "
}
