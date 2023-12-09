use crate::utils::parsers::*;
use ahash::AHashSet;

fn parse_mappings(input: &str) -> (Vec<(&str, &str)>, &str) {
    separated_pair(lines(sep2(alpha1, " => ")), "\n\n", alpha1).read(input)
}

fn single_repls<'a>(src: &'a str, k: &'a str, v: &'a str) -> impl Iterator<Item = String> + 'a {
    src.match_indices(k).map(|(i, _)| {
        let mut s = src.to_string();
        s.replace_range(i..i + k.len(), v);
        s
    })
}

pub fn part1(input: &str) -> usize {
    let (mappings, s) = parse_mappings(input);
    mappings
        .into_iter()
        .flat_map(|(k, v)| single_repls(s, k, v))
        .collect::<AHashSet<_>>()
        .len()
}

pub fn part2(input: &str) -> usize {
    let mol = parse_mappings(input).1;
    mol.matches(|c: char| c.is_ascii_uppercase()).count()
        - (mol.matches("Rn").count() + mol.matches("Ar").count())
        - 2 * mol.matches('Y').count()
        - 1
}
