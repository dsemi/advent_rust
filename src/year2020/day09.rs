use itertools::Itertools;
use std::cmp::Ordering::*;

fn find_first_invalid(ns: &[i64]) -> Option<i64> {
    (25..ns.len()).find_map(|n| {
        (!ns[n - 25..n].iter().combinations(2).any(|combo| combo.into_iter().sum::<i64>() == ns[n]))
            .then(|| ns[n])
    })
}

pub fn part1(ns: Vec<i64>) -> Option<i64> {
    find_first_invalid(&ns)
}

pub fn part2(ns: Vec<i64>) -> Option<i64> {
    let n = find_first_invalid(&ns)?;
    let (mut lo, mut hi, mut acc) = (0, 0, 0);
    while acc != n {
        match acc.cmp(&n) {
            Less => {
                acc += ns[hi];
                hi += 1;
            }
            Greater => {
                acc -= ns[lo];
                lo += 1;
            }
            Equal => unreachable!(),
        }
    }
    let arr = &ns[lo..hi];
    Some(arr.iter().min()? + arr.iter().max()?)
}
