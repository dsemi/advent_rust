use crate::utils::parsers::*;
use crate::utils::*;

fn parse(input: &str) -> (Vec<Interval<u64>>, Vec<u64>) {
    let (mut rs, mut ids) = separated_pair(lines(sep2(u64, '-')), "\n\n", lines(u64)).read(input);
    rs.sort_unstable_by_key(|&(lo, _)| lo);
    ids.sort_unstable();
    let mut intervals = rs.into_iter().map(|(lo, hi)| Interval::new(lo, hi + 1));
    let mut comp = vec![intervals.next().unwrap()];
    for next in intervals {
        match comp.last().unwrap().union(&next) {
            Some(u) => *comp.last_mut().unwrap() = u,
            None => comp.push(next),
        }
    }
    (comp, ids)
}

pub fn part1(input: &str) -> usize {
    let (intervals, ids) = parse(input);
    let find = |x| ids.binary_search(&x).unwrap_or_else(|e| e);
    intervals.into_iter().map(|i| find(i.hi) - find(i.lo)).sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input).0.into_iter().map(|i| i.len()).sum()
}
