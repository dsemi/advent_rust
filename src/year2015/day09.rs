use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn all_path_distances(input: &str) -> Vec<i32> {
    let mut m: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let (k1, k2, v) = (&cap[1], &cap[2], cap[3].parse().unwrap());
        m.entry(k1.to_string())
            .or_insert_with(HashMap::new)
            .insert(k2.to_string(), v);
        m.entry(k2.to_string())
            .or_insert_with(HashMap::new)
            .insert(k1.to_string(), v);
    }
    m.keys()
        .permutations(m.len())
        .map(|perm| {
            perm.windows(2)
                .map(|p| m.get(&p[0].clone()).unwrap().get(&p[1].clone()).unwrap())
                .sum()
        })
        .collect()
}

pub fn part1(input: &str) -> Option<i32> {
    all_path_distances(input).into_iter().min()
}

pub fn part2(input: &str) -> Option<i32> {
    all_path_distances(input).into_iter().max()
}
