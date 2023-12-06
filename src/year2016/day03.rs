use crate::utils::parsers::*;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(int).collect())
        .collect()
}

fn valid(sides: &[u32]) -> bool {
    sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0]
}

pub fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(|v| valid(v)).count()
}

pub fn part2(input: &str) -> usize {
    let t = &parse(input);
    (0..t.len())
        .step_by(3)
        .flat_map(|i| (0..3).map(move |j| valid(&[t[i][j], t[i + 1][j], t[i + 2][j]]) as usize))
        .sum()
}
