use crate::ocr::*;
use crate::utils::*;

fn run(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split_whitespace()
        .map(|tok| match tok {
            "addx" | "noop" => 0,
            _ => tok.parse().unwrap(),
        })
        .good_scan(1, |x, d| x + d)
        .take(240)
}

pub fn part1(input: &str) -> i32 {
    run(input)
        .enumerate()
        .filter_map(|(c, x)| ((c + 1) % 40 == 20).then(|| (c + 1) as i32 * x))
        .sum()
}

pub fn part2(input: &str) -> String {
    let mut res = String::new();
    for (c, x) in run(input).enumerate() {
        let m = c as i32 % 40;
        if m == 0 {
            res.push('\n');
        }
        res.push(if (m - x).abs() <= 1 { '#' } else { ' ' });
    }
    parse_letters(&res, None)
}
