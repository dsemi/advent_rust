use crate::utils::parsers2::*;
use std::cmp::Ordering::*;

fn solve(mut n: u32, f: fn(u8) -> bool) -> bool {
    let mut prev = n % 10;
    let mut c = 1;
    let mut b = false;
    n /= 10;
    while n != 0 {
        let m = n % 10;
        match m.cmp(&prev) {
            Equal => c += 1,
            Greater => return false,
            Less => {
                b = b || f(c);
                c = 1;
                prev = m;
            }
        }
        n /= 10;
    }
    b || f(c)
}

fn num_valid(input: &str, f: fn(u8) -> bool) -> usize {
    let (start, end) = sep_tuple2(u32, '-').read(input);
    (start..=end).filter(|&v| solve(v, f)).count()
}

pub fn part1(input: &str) -> usize {
    num_valid(input, |x| x >= 2)
}

pub fn part2(input: &str) -> usize {
    num_valid(input, |x| x == 2)
}
