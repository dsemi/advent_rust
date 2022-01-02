use mod_exp::mod_exp;
use regex::Regex;

pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"\d+").unwrap();
    let v: Vec<i64> = re
        .find_iter(input)
        .map(|x| x.as_str().parse().unwrap())
        .collect();
    let (r, c) = (v[0], v[1]);
    let n = r + c - 1;
    let index = n * (n - 1) / 2 + c - 1;
    mod_exp(252533, index, 33554393) * 20151125 % 33554393
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
