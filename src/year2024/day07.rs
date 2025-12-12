use crate::utils::parsers::*;

fn validate(curr: i64, ns: &[i64], p2: bool) -> bool {
    let (&n, ns) = ns.split_last().unwrap();
    if ns.is_empty() {
        return curr == n;
    }
    let m = 10_i64.pow(n.ilog10() + 1);
    (p2 && curr % m == n && validate(curr / m, ns, p2))
        || (curr % n == 0 && validate(curr / n, ns, p2))
        || (curr >= n && validate(curr - n, ns, p2))
}

fn kvs(i: &mut &str) -> Result<(i64, Vec<i64>)> {
    separated_pair(i64, ": ", spaced(i64)).parse_next(i)
}

pub fn part1(input: &str) -> i64 {
    lines_iter(input, kvs).filter_map(|(k, v)| validate(k, &v, false).then_some(k)).sum()
}

pub fn part2(input: &str) -> i64 {
    lines_iter(input, kvs).filter_map(|(k, v)| validate(k, &v, true).then_some(k)).sum()
}
