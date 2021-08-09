use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use regex::{Captures, Regex};

fn parse_mappings(input: &str) -> (&str, Vec<(&str, &str)>) {
    let v: Vec<_> = input.split("\n\n").collect();
    (
        v[1],
        v[0].lines()
            .map(|line| {
                let v: Vec<_> = line.split(" => ").collect();
                (v[0], v[1])
            })
            .collect(),
    )
}

fn single_replacements(src: &str, k: &str, v: &str) -> Vec<String> {
    let re = Regex::new(k).unwrap();
    re.find_iter(src)
        .map(|m| {
            let mut s = src.to_string();
            s.replace_range(m.range(), v);
            s
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let (s, mappings) = parse_mappings(input);
    mappings
        .into_iter()
        .flat_map(|(k, v)| single_replacements(s, k, v))
        .collect::<AHashSet<_>>()
        .len()
}

pub fn part2(input: &str) -> i32 {
    let (s, mappings) = parse_mappings(input);
    let mut mol: String = s.chars().rev().collect();
    let reps: AHashMap<String, String> = mappings
        .into_iter()
        .map(|(a, b)| (b.chars().rev().collect(), a.chars().rev().collect()))
        .collect();
    let re = Regex::new(&reps.keys().join("|")).unwrap();
    let mut count = 0;
    while mol != "e" {
        mol = re
            .replace(&mol, |caps: &Captures| &reps[&caps[0]])
            .into();
        count += 1;
    }
    count
}
