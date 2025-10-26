use crate::utils::parsers::*;
use hashbrown::HashMap;

fn parse_orbits(input: &str) -> impl Iterator<Item = (&str, &str)> {
    lines_iter(input, sep2(alphanumeric1, ')'))
}

pub fn part1(input: &str) -> usize {
    let mut t = HashMap::new();
    for (k, v) in parse_orbits(input) {
        let e = t.entry(k.to_string()).or_insert_with(Vec::new);
        (*e).push(v);
    }
    let mut depth = 0;
    let mut keys = vec!["COM"];
    let mut result = 0;
    while !keys.is_empty() {
        result += depth * keys.len();
        keys = keys.into_iter().flat_map(|x| t.get(x).unwrap_or(&vec![]).clone()).collect();
        depth += 1;
    }
    result
}

fn path_from_com<'a>(t: &'a HashMap<&str, &str>, key: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut v = t.get(key);
    while let Some(&k) = v {
        result.push(k);
        v = t.get(k);
    }
    result.reverse();
    result
}

pub fn part2(input: &str) -> Option<usize> {
    let t = parse_orbits(input).map(|(k, v)| (v, k)).collect::<HashMap<_, _>>();
    let xs = path_from_com(&t, "YOU");
    let ys = path_from_com(&t, "SAN");
    xs.iter()
        .zip(ys.iter())
        .enumerate()
        .find(|(_, (x, y))| x != y)
        .map(|(i, _)| xs.len() + ys.len() - 2 * i)
}
