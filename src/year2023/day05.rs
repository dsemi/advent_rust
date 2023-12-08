use crate::utils::parsers2::*;
use crate::utils::*;
use std::cmp::Ordering::*;

fn parse_seeds(i: &mut &str) -> PResult<Vec<i64>> {
    preceded("seeds: ", separated(1.., i64, space1)).parse_next(i)
}

fn parse_map(input: &str) -> Vec<(Interval, i64)> {
    let mut result = Vec::new();
    for line in input.lines().skip(1) {
        let (dest, src, len) = sep_tuple3(i64, space1).read(line);
        result.push((Interval::new(src, src + len), dest - src))
    }
    result.sort_unstable_by_key(|(i, _)| i.lo);
    result
}

pub fn part1(input: &str) -> i64 {
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
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds.read(parts.next().unwrap());
    let seed_intervals: Vec<_> = seeds
        .chunks(2)
        .map(|ns| Interval::new(ns[0], ns[0] + ns[1]))
        .collect();
    let maps: Vec<_> = parts.map(parse_map).collect();
    seed_intervals
        .into_iter()
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
        .unwrap()
}
