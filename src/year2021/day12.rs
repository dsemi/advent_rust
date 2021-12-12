use ahash::{AHashMap, AHashSet};

fn parse(input: &str) -> AHashMap<&str, Vec<&str>> {
    let mut m = AHashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        m.entry(a).or_insert_with(Vec::new).push(b);
        m.entry(b).or_insert_with(Vec::new).push(a);
    }
    m
}

fn dfs<'a>(
    visited: &mut AHashSet<&'a str>,
    m: &AHashMap<&'a str, Vec<&'a str>>,
    k: &'a str,
    mut double: bool,
) -> usize {
    if k == "end" {
        return 1;
    }
    let mut ins = false;
    if k.chars().next().unwrap().is_lowercase() {
        ins = visited.insert(k);
        if !ins {
            if double || k == "start" {
                return 0;
            }
            double = true;
        }
    }
    let sum = m[k]
        .iter()
        .map(|child| dfs(visited, m, child, double))
        .sum();
    if ins {
        visited.remove(k);
    }
    sum
}

pub fn part1(input: &str) -> usize {
    dfs(&mut AHashSet::new(), &parse(input), "start", true)
}

pub fn part2(input: &str) -> usize {
    dfs(&mut AHashSet::new(), &parse(input), "start", false)
}
