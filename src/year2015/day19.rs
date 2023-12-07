use crate::utils::parsers::*;
use ahash::AHashSet;
use regex::Regex;

fn parse_mappings(input: &str) -> (&str, Vec<(&str, &str)>) {
    let v: Vec<_> = input.split("\n\n").collect();
    (v[1], lines(sep_tuple2(tag(" => "), alpha1)).read(v[0]))
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

pub fn part2(input: &str) -> usize {
    let mol = parse_mappings(input).0;
    mol.matches(|c: char| c.is_ascii_uppercase()).count()
        - (mol.matches("Rn").count() + mol.matches("Ar").count())
        - 2 * mol.matches('Y').count()
        - 1
}
