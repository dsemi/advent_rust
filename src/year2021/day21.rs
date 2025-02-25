use crate::utils::parsers::*;
use hashbrown::HashMap;
use std::cmp::{max, min};

fn parse(i: &mut &str) -> PResult<(u64, u64)> {
    let (p1, p2) =
        sep2(preceded(("Player ", u8, " starting position: "), u64), '\n').parse_next(i)?;
    Ok((p1 - 1, p2 - 1))
}

pub fn part1(input: &str) -> u64 {
    let (mut p1, mut p2) = parse.read(input);
    let (mut p1_score, mut p2_score) = (0, 0);
    let die = &mut (1..=100).cycle();
    let mut n = 0;
    while p2_score < 1000 {
        p1 = (p1 + die.take(3).sum::<u64>()) % 10;
        p1_score += p1 + 1;
        n += 3;
        std::mem::swap(&mut p1, &mut p2);
        std::mem::swap(&mut p1_score, &mut p2_score);
    }
    min(p1_score, p2_score) * n
}

const PROBS: &[(u64, u64)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type Cache = HashMap<(u64, u64, u64, u64), (u64, u64)>;

fn solve(cache: &mut Cache, p1: u64, p2: u64, s1: u64, s2: u64) -> (u64, u64) {
    if s1 >= 21 {
        return (1, 0);
    }
    if s2 >= 21 {
        return (0, 1);
    }
    if let Some(v) = cache.get(&(p1, p2, s1, s2)) {
        return *v;
    }
    let mut ans = (0, 0);
    for (d, n) in PROBS {
        let new_p1 = (p1 + d) % 10;
        let (x1, y1) = solve(cache, p2, new_p1, s2, s1 + new_p1 + 1);
        ans = (ans.0 + n * y1, ans.1 + n * x1);
    }
    cache.insert((p1, p2, s1, s2), ans);
    ans
}

pub fn part2(input: &str) -> u64 {
    let (p1, p2) = parse.read(input);
    let (x, y) = solve(&mut HashMap::new(), p1, p2, 0, 0);
    max(x, y)
}
