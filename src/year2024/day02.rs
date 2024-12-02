use crate::utils::parsers::*;

fn valid(ns: &[u8]) -> bool {
    let ord = ns[0].cmp(&ns[1]);
    ns.windows(2).all(|w| w[0].cmp(&w[1]) == ord && (1..=3).contains(&w[0].abs_diff(w[1])))
}

pub fn part1(input: &str) -> usize {
    input.lines().map(|line| spaced(u8).read(line)).filter(|ns| valid(ns)).count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| spaced(u8).read(line))
        .filter(|ns| {
            valid(ns)
                || (0..ns.len()).any(|i| {
                    let mut ns = ns.clone();
                    ns.remove(i);
                    valid(&ns)
                })
        })
        .count()
}
