use crate::utils::{held_karp, UniqueIdx};
use std::cmp::max;

fn parse_happiness(input: &str) -> Vec<Vec<i32>> {
    let mut ui = UniqueIdx::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l = (input.lines().count() as f32).sqrt() as usize + 1;
    for _ in 0..l {
        result.push(vec![0; l]);
    }
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let (p1, n, p2) = (
            parts[0],
            if parts[2] == "gain" {
                parts[3].parse::<i32>().unwrap()
            } else {
                -parts[3].parse::<i32>().unwrap()
            },
            &parts[10][..parts[10].len() - 1],
        );
        result[ui.idx(p1)][ui.idx(p2)] += n;
        result[ui.idx(p2)][ui.idx(p1)] += n;
    }
    result
}

pub fn part1(input: &str) -> Option<i32> {
    held_karp(&parse_happiness(input), max)
}

pub fn part2(input: &str) -> Option<i32> {
    let mut adj = parse_happiness(input);
    adj.iter_mut().for_each(|row| row.push(0));
    adj.push(vec![0; adj.len()]);
    held_karp(&adj, max)
}
