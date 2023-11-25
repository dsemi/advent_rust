use crate::utils::*;
use num::Integer;

fn parse_firewall(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().map(|line| {
        let (a, b) = line.split_once(": ").unwrap();
        (a.parse().unwrap(), 2 * b.parse::<i64>().unwrap() - 2)
    })
}

pub fn part1(input: &str) -> i64 {
    parse_firewall(input)
        .filter_map(|(a, b)| (a % b == 0).then_some(a * (b + 2) / 2))
        .sum()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut lcm = 1;
    parse_firewall(input)
        .fold(vec![1], |curr, (d, p)| {
            let old_lcm = replace_with(&mut lcm, |x| p.lcm(x)) as usize;
            (0..lcm)
                .step_by(old_lcm)
                .flat_map(|extra| curr.iter().map(move |delay| delay + extra))
                .filter(|&de| (de + d) % p != 0)
                .collect()
        })
        .first()
        .copied()
}
