use crate::utils::parsers::*;
use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;

struct Input {
    rules: Vec<(String, i64, i64, i64, i64)>,
    yours: Vec<i64>,
    tix: Vec<Vec<i64>>,
}

fn parse_rules(s: &str) -> Input {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let rules: Vec<(String, i64, i64, i64, i64)> = parts[0]
        .lines()
        .map(|line| scanf!(line, "{[^:]}: {}-{} or {}-{}", String, i64, i64, i64, i64).unwrap())
        .collect();
    let yours = preceded(tag("your ticket:\n"), list(i64)).read(parts[1]);
    let others = preceded(tag("nearby tickets:\n"), lines(list(i64))).read(parts[2]);
    Input {
        rules,
        yours,
        tix: others,
    }
}

fn invalid_values(rules: &[(String, i64, i64, i64, i64)], ticket: &[i64]) -> Vec<i64> {
    ticket
        .iter()
        .filter(|&field| {
            !rules
                .iter()
                .any(|(_, a, b, c, d)| a <= field && field <= b || c <= field && field <= d)
        })
        .copied()
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let Input {
        rules,
        yours: _,
        tix,
    } = parse_rules(input);
    tix.iter().flat_map(|t| invalid_values(&rules, t)).sum()
}

pub fn part2(input: &str) -> i64 {
    let Input { rules, yours, tix } = parse_rules(input);
    let tix: Vec<Vec<i64>> = tix
        .into_iter()
        .filter(|t| invalid_values(&rules, t).is_empty())
        .collect();
    let mut poss = vec![];
    for _ in 0..yours.len() {
        poss.push(rules.clone());
    }
    for t in tix {
        poss = poss
            .into_iter()
            .zip(t)
            .map(|(p, f)| {
                p.into_iter()
                    .filter(|&(_, a, b, c, d)| a <= f && f <= b || c <= f && f <= d)
                    .collect()
            })
            .collect();
    }
    let mut poss_set: Vec<AHashSet<String>> = poss
        .into_iter()
        .map(|p| p.into_iter().map(|x| x.0).collect())
        .collect();
    while !poss_set.iter().all(|p| p.len() == 1) {
        let ones: AHashSet<String> = poss_set
            .iter()
            .filter(|p| p.len() == 1)
            .flat_map(|p| p.iter().map(|x| x.to_string()))
            .collect();
        poss_set = poss_set
            .into_iter()
            .map(|p| {
                if p.len() == 1 {
                    p
                } else {
                    p.difference(&ones).map(|x| x.to_string()).collect()
                }
            })
            .collect();
    }
    poss_set
        .into_iter()
        .map(|p| p.into_iter().next().unwrap())
        .zip(yours)
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| x.1)
        .product()
}
