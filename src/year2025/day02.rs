use crate::utils::parsers::*;
use itertools::Itertools;

struct Range {
    lo: u64,
    hi: u64,
    step: u64,
}

const fn calc(digits: u32, seg_size: u32) -> Range {
    let step = (10_u64.pow(digits) - 1) / (10_u64.pow(seg_size) - 1);
    let lo = step * 10_u64.pow(seg_size - 1);
    let hi = step * (10_u64.pow(seg_size) - 1);
    Range { lo, hi, step }
}

fn solve(intervals: &[(u64, u64)], ranges: &[Range]) -> u64 {
    let mut result = 0;
    for (range, &(lo, hi)) in ranges.iter().cartesian_product(intervals) {
        let lo = lo.next_multiple_of(range.step).max(range.lo);
        let hi = hi.min(range.hi);
        if lo <= hi {
            let n = (hi - lo) / range.step;
            result += lo * (n + 1) + range.step * n * (n + 1) / 2;
        }
    }
    result
}

const HALVES: [Range; 5] = [calc(2, 1), calc(4, 2), calc(6, 3), calc(8, 4), calc(10, 5)];

pub fn part1(input: &str) -> u64 {
    let ints = list(sep2(u64, '-')).read(input);
    solve(&ints, &HALVES)
}

const NEW: [Range; 6] = [calc(3, 1), calc(5, 1), calc(6, 2), calc(7, 1), calc(9, 3), calc(10, 2)];
const OVERLAP: [Range; 2] = [calc(6, 1), calc(10, 1)];

pub fn part2(input: &str) -> u64 {
    let ints = list(sep2(u64, '-')).read(input);
    solve(&ints, &HALVES) + solve(&ints, &NEW) - solve(&ints, &OVERLAP)
}
