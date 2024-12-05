use std::cmp::Ordering::{self, *};

use crate::utils::parsers::*;

fn parse(input: &str) -> ([[Ordering; 100]; 100], impl Iterator<Item = Vec<usize>> + use<'_>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let mut rule_map = [[Equal; 100]; 100];
    for (a, b) in rules.lines().map(|line| sep2(usize, '|').read(line)) {
        rule_map[a][b] = Less;
        rule_map[b][a] = Greater;
    }
    (rule_map, pages.lines().map(|line| list(usize).read(line)))
}

pub fn part1(input: &str) -> usize {
    let (rules, pages) = parse(input);
    pages
        .filter(|page| page.is_sorted_by(|&a, &b| rules[a][b] == Less))
        .map(|page| page[page.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (rules, pages) = parse(input);
    pages
        .filter(|page| !page.is_sorted_by(|&a, &b| rules[a][b] == Less))
        .map(|mut page| {
            let middle = page.len() / 2;
            *page.select_nth_unstable_by(middle, |&a, &b| rules[a][b]).1
        })
        .sum()
}
