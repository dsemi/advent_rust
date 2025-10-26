use crate::utils::parsers::*;
use crate::utils::{UniqueIdx, held_karp};
use std::cmp::max;

fn parser<'a>(i: &mut &'a str) -> Result<(&'a str, i32, &'a str)> {
    (
        alpha1,
        " would ",
        alt(("gain ".value(1), "lose ".value(-1))),
        i32,
        " happiness units by sitting next to ",
        alpha1,
        '.',
    )
        .map(|(p1, _, sgn, n, _, p2, _)| (p1, sgn * n, p2))
        .parse_next(i)
}

fn parse_happiness(input: &str) -> Vec<Vec<i32>> {
    let mut ui = UniqueIdx::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l = (input.lines().count() as f32).sqrt() as usize + 1;
    for _ in 0..l {
        result.push(vec![0; l]);
    }
    for line in input.lines() {
        let (p1, n, p2) = parser.read(line);
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
