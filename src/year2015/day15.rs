use crate::utils::parsers::*;
use crate::utils::Partitions;
use std::cmp::max;
use streaming_iterator::StreamingIterator;

fn ingredients(s: &str) -> Vec<Vec<i32>> {
    lines(preceded((alpha1, ": "), list(preceded((alpha1, ' '), i32)))).read(s)
}

fn max_score(total: i32, cal_pred: fn(i32) -> bool, ings: Vec<Vec<i32>>) -> Option<i32> {
    Partitions::new(ings.len(), total)
        .filter_map_deref(|ms| {
            let mut v = [0; 5];
            for i in 0..ms.len() {
                for (j, e) in v.iter_mut().enumerate() {
                    *e += ms[i] * ings[i][j];
                }
            }
            cal_pred(v[4]).then(|| v[..4].iter().map(|&x| max(0, x)).product())
        })
        .max()
}

pub fn part1(input: &str) -> Option<i32> {
    max_score(100, |_| true, ingredients(input))
}

pub fn part2(input: &str) -> Option<i32> {
    max_score(100, |x| x == 500, ingredients(input))
}
