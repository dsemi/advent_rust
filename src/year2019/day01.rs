use itertools::iterate;

pub fn part1(ns: Vec<i64>) -> i64 {
    ns.into_iter().map(|n| n / 3 - 2).sum()
}

pub fn part2(ns: Vec<i64>) -> i64 {
    ns.into_iter()
        .flat_map(|n| iterate(n / 3 - 2, |fuel| fuel / 3 - 2).take_while(|&fuel| fuel > 0))
        .sum()
}
