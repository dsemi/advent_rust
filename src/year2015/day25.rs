use crate::utils::parsers::*;
use crate::utils::*;
use num_traits::Pow;

pub fn part1(input: &str) -> i64 {
    let input = input.replace(|c: char| !c.is_ascii_digit(), " ");
    let (r, c) = preceded(space0, sep_tuple2(space1, i64)).read(&input);
    let n = r + c - 1;
    let index = n * (n - 1) / 2 + c - 1;
    (Mod::<33554393>(252533).pow(index) * Mod(20151125)).0
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
