use ndarray::prelude::*;
use regex::Regex;
use std::cmp::max;
use std::iter::once;

fn partitions(n: usize, t: i32) -> Box<dyn Iterator<Item = Vec<i32>>> {
    if n == 1 {
        return Box::new(once(vec![t]));
    }
    Box::new((0..=t).flat_map(move |x| {
        partitions(n - 1, t - x).map(move |mut xs| {
            xs.insert(0, x);
            xs
        })
    }))
}

fn parse_ingredients(s: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"-?\d+").unwrap();
    s.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|n| n.as_str().parse().unwrap())
                .collect()
        })
        .collect()
}

fn scores(total: i32, cal_pred: fn(i32) -> bool, ings: Vec<Vec<i32>>) -> impl Iterator<Item = i32> {
    partitions(ings.len(), total)
        .map(move |ms| {
            ms.iter()
                .zip(ings.iter())
                .map(|(n, i)| *n * arr1(i))
                .fold(Array::zeros(5), |acc, x| acc + x)
        })
        .filter(move |v| cal_pred(v[4]))
        .map(|v| v.slice(s![..4]).map(|&x| max(0, x)).product())
}

pub fn part1(input: &str) -> Option<i32> {
    scores(100, |_| true, parse_ingredients(input)).max()
}

pub fn part2(input: &str) -> Option<i32> {
    scores(100, |x| x == 500, parse_ingredients(input)).max()
}
