use itertools::Itertools;
use std::cmp::Ordering::*;

fn find_first_invalid(ns: &[i64]) -> i64 {
    let mut n = 25;
    loop {
        if !ns[n - 25..n]
            .iter()
            .combinations(2)
            .any(|combo| combo.into_iter().sum::<i64>() == ns[n])
        {
            return ns[n];
        }
        n += 1;
    }
}

pub fn part1(input: Vec<i64>) -> i64 {
    find_first_invalid(&input)
}

pub fn part2(ns: Vec<i64>) -> Option<i64> {
    let n = find_first_invalid(&ns);
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
            Equal => (),
        }
    }
    let arr = &ns[lo..hi];
    Some(arr.iter().min()? + arr.iter().max()?)
}
