use itertools::Itertools;
use std::collections::HashMap;

fn all_path_distances<'a>(input: &str) -> Vec<i32> {
    let mut m: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let (k1, k2, v) = (parts[0], parts[2], parts[4].parse().unwrap());
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
                .map(|p| m[p[0]][p[1]])
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
