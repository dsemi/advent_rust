use crate::utils::*;

const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn solve<'a>(input: &str, repls: impl Iterator<Item = (usize, &'a &'a str)>) -> usize {
    let repls: Vec<_> = DIGITS.iter().enumerate().chain(repls).collect();
    input
        .lines()
        .map(|line| {
            let tens = tails(line)
                .find_map(|s| repls.iter().find(|x| s.starts_with(x.1)).map(|x| x.0 + 1))
                .unwrap();
            let ones = inits(line)
                .find_map(|s| repls.iter().find(|x| s.ends_with(x.1)).map(|x| x.0 + 1))
                .unwrap();
            10 * tens + ones
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, std::iter::empty())
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part2(input: &str) -> usize {
    solve(input, WORDS.iter().enumerate())
}
