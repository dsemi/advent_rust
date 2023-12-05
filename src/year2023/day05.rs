use crate::utils::parsers::*;
use crate::utils::*;
use std::cmp::Ordering::*;
use std::cmp::{max, min};

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(space1, i64))(input)
}

fn parse_map(input: &str) -> Vec<(Interval, i64)> {
    let mut result = Vec::new();
    for line in input.lines().skip(1) {
        let (dest, src, len) = separated_triplet(space1, i64)(line).unwrap().1;
        result.push((Interval::new(src, src + len), dest - src))
    }
    result.sort_unstable_by_key(|(i, _)| i.lo);
    result
}

pub fn part1(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap()).unwrap().1;
    let maps: Vec<_> = parts.map(|pt| parse_map(pt)).collect();
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.binary_search_by(|(filter, _)| {
                    filter
                        .contains(seed)
                        .then_some(Equal)
                        .unwrap_or(filter.lo.cmp(&seed))
                })
                .map(|idx| seed + map[idx].1)
                .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap()).unwrap().1;
    let seed_intervals: Vec<_> = seeds
        .chunks(2)
        .map(|ns| Interval::new(ns[0], ns[0] + ns[1]))
        .collect();
    let maps: Vec<_> = parts.map(|pt| parse_map(pt)).collect();
    seed_intervals
        .into_iter()
        .flat_map(|interval| {
            maps.iter().fold(vec![interval], |mut intervals, map| {
                let mut result = Vec::new();
                for &(src, offset) in map.iter() {
                    let mut next_intervals = Vec::new();
                    intervals.into_iter().for_each(|int| {
                        let before = Interval::new(int.lo, min(int.hi, src.lo));
                        let inter = int.intersect(&src);
                        let after = Interval::new(max(int.lo, src.hi), int.hi);
                        if before.valid() {
                            next_intervals.push(before)
                        }
                        if inter.valid() {
                            result.push(inter + offset)
                        }
                        if after.valid() {
                            next_intervals.push(after)
                        }
                    });
                    intervals = next_intervals;
                }
                result.extend(intervals);
                result
            })
        })
        .map(|int| int.lo)
        .min()
        .unwrap()
}
