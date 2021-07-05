use std::collections::HashSet;

pub fn part1(input: &str) -> i64 {
    input.lines().map(|x| x.parse::<i64>().unwrap()).sum()
}

pub fn part2(input: &str) -> Option<i64> {
    let ns: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut s = HashSet::new();
    ns.into_iter()
        .cycle()
        .scan(0, |st, x| {
            *st += x;
            Some(*st)
        })
        .filter(|&x| !s.insert(x))
        .next()
}
