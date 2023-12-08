use crate::utils::parsers::*;

fn nums(i: &str) -> impl Iterator<Item = u64> + '_ {
    i.split_whitespace().skip(1).map(u64::read)
}

fn race(time: u64, dist: u64) -> u64 {
    // hold^2 - hold*time + dist = 0
    let root = ((time * time - 4 * dist) as f64).sqrt();
    let start = (time as f64 - root) / 2.0;
    let end = (time as f64 + root) / 2.0;
    end.ceil() as u64 - start.floor() as u64 - 1
}

pub fn part1(input: &str) -> u64 {
    let (times, dists) = input.split_once('\n').unwrap();
    nums(times)
        .zip(nums(dists))
        .map(|(time, win_dist)| race(time, win_dist))
        .product()
}

fn squish(i: &str) -> String {
    i.chars().filter(char::is_ascii_digit).collect()
}

pub fn part2(input: &str) -> u64 {
    let (times, dists) = input.split_once('\n').unwrap();
    race(squish(times).u64(), squish(dists).u64())
}
