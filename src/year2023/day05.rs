use itertools::Itertools;

use crate::utils::parsers::*;
use crate::utils::*;
use std::cmp::Ordering::*;

fn parse_seeds(i: &mut &str) -> PResult<Vec<u32>> {
    preceded("seeds: ", separated(1.., u32, space1)).parse_next(i)
}

fn parse_map(input: &str) -> Vec<(Interval<u32>, u32)> {
    input
        .lines()
        .skip(1)
        .map(|line| sep3(u32, space1).read(line))
        .map(|(dest, src, len)| (Interval::new(src, src + len), dest - src))
        .sorted_unstable_by_key(|(i, _)| i.lo)
        .collect()
}

pub fn part1(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds.read(parts.next().unwrap());
    let maps: Vec<_> = parts.map(parse_map).collect();
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.binary_search_by(|(filter, _)| {
                    if filter.contains(seed) {
                        Equal
                    } else {
                        filter.lo.cmp(&seed)
                    }
                })
                .map(|idx| seed + map[idx].1)
                .unwrap_or(seed)
            })
        })
        .min()
}

pub fn part2(input: &str) -> Option<u32> {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds.read(parts.next().unwrap());
    let maps: Vec<_> = parts.map(parse_map).collect();
    seeds
        .chunks(2)
        .map(|ns| Interval::new(ns[0], ns[0] + ns[1]))
        .flat_map(|interval| {
            maps.iter().fold(vec![interval], |mut intervals, map| {
                let mut result = Vec::new();
                for &(src, offset) in map.iter() {
                    let mut next_intervals = Vec::new();
                    intervals.into_iter().for_each(|int| {
                        let inter = int.intersect(&src);
                        if inter.valid() {
                            result.push(inter + offset)
                        }
                        next_intervals.extend(int - inter);
                    });
                    intervals = next_intervals;
                }
                result.extend(intervals);
                result
            })
        })
        .map(|int| int.lo)
        .min()
}
