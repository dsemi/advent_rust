use ahash::{AHashMap, AHashSet};
use std::cmp::Ordering::*;

use crate::utils::parsers::*;

fn parse(input: &str) -> (AHashMap<u32, AHashSet<u32>>, impl Iterator<Item = Vec<u32>> + use<'_>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut rule_map = AHashMap::new();
    for (k, v) in rules.lines().map(|line| sep2(u32, '|').read(line)) {
        rule_map.entry(k).or_insert_with(AHashSet::new).insert(v);
    }
    (rule_map, pages.lines().map(|line| list(u32).read(line)))
}

pub fn part1(input: &str) -> u32 {
    let (rules, pages) = parse(input);
    pages
        .filter(|page| page.is_sorted_by(|a, b| rules[a].contains(b)))
        .map(|page| page[page.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (rules, pages) = parse(input);
    pages
        .filter(|page| !page.is_sorted_by(|a, b| rules[a].contains(b)))
        .map(|mut page| {
            page.sort_by(|a, b| if rules[a].contains(b) { Less } else { Greater });
            page[page.len() / 2]
        })
        .sum()
}
