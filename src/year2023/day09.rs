use crate::utils::parsers::*;

fn extrapolate(ns: Vec<i64>) -> i64 {
    if ns.iter().all(|&n| n == 0) {
        return 0;
    }
    ns.last().unwrap() + extrapolate(ns.windows(2).map(|w| w[1] - w[0]).collect())
}

fn solve(input: &str, part2: bool) -> i64 {
    let mut vals = lines(spaced(i64)).read(input);
    if part2 {
        vals.iter_mut().for_each(|row| row.reverse());
    }
    vals.into_iter().map(extrapolate).sum()
}

pub fn part1(input: &str) -> i64 {
    solve(input, false)
}

pub fn part2(input: &str) -> i64 {
    solve(input, true)
}
