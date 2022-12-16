use crate::utils::UniqueIdx;
use itertools::Itertools;

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

fn max_happiness(d: Vec<Vec<i32>>, p2: bool) -> Option<i32> {
    (0..d.len())
        .permutations(d.len())
        .map(|perm| {
            let mut curr = if !p2 {
                d[perm[0]][perm[d.len() - 1]]
            } else {
                0
            };
            for i in 1..d.len() {
                curr += d[perm[i]][perm[i - 1]];
            }
            curr
        })
        .max()
}

pub fn part1(input: &str) -> Option<i32> {
    max_happiness(parse_happiness(input), false)
}

pub fn part2(input: &str) -> Option<i32> {
    max_happiness(parse_happiness(input), true)
}
