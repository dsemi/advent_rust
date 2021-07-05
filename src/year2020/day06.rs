use std::collections::HashSet;

fn solve(f: fn(HashSet<char>, HashSet<char>) -> HashSet<char>, s: &str) -> usize {
    s.split("\n\n")
        .map(|group| {
            let mut iter = group
                .split_whitespace()
                .map(|x| x.chars().collect::<HashSet<_>>());
            iter.next().map(|set| iter.fold(set, f)).unwrap().len()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(|a, b| &a | &b, input)
}

pub fn part2(input: &str) -> usize {
    solve(|a, b| &a & &b, input)
}
