use itertools::{Itertools, MinMaxResult::MinMax, iproduct};

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let ns: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            match ns.iter().minmax() {
                MinMax(mn, mx) => mx - mn,
                _ => panic!("No"),
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let ns: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            iproduct!(ns.iter(), ns.iter())
                .filter_map(|(x, y)| (x != y && x % y == 0).then(|| x / y))
                .next()
                .unwrap()
        })
        .sum()
}
