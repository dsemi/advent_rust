use crate::utils::parsers::*;

fn validate(curr: i64, ns: &[i64], p2: bool) -> bool {
    if ns.is_empty() {
        return curr == 0;
    }
    if curr < 0 {
        return false;
    }
    let n = ns[ns.len() - 1];
    let m = 10_i64.pow(n.ilog10() + 1);
    (p2 && curr % m == n && validate(curr / m, &ns[..ns.len() - 1], p2))
        || (curr % n == 0 && validate(curr / n, &ns[..ns.len() - 1], p2))
        || validate(curr - n, &ns[..ns.len() - 1], p2)
}

pub fn part1(input: &str) -> i64 {
    lines_iter(input, separated_pair(i64, ": ", spaced(i64)))
        .filter_map(|(k, v)| validate(k, &v, false).then_some(k))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    lines_iter(input, separated_pair(i64, ": ", spaced(i64)))
        .filter_map(|(k, v)| validate(k, &v, true).then_some(k))
        .sum()
}
