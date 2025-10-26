use crate::utils::parsers::*;
use hashbrown::HashSet;

struct Input<'a> {
    rules: Vec<(&'a str, i64, i64, i64, i64)>,
    yours: Vec<i64>,
    tix: Vec<Vec<i64>>,
}

fn rule<'a>(i: &mut &'a str) -> Result<(&'a str, i64, i64, i64, i64)> {
    let loc = terminated(take_till(0.., |c| c == ':'), ": ").parse_next(i)?;
    let (a1, _, a2, _, b1, _, b2) = (i64, '-', i64, " or ", i64, '-', i64).parse_next(i)?;
    Ok((loc, a1, a2, b1, b2))
}

fn parse_rules(s: &str) -> Input<'_> {
    separated_triplet(
        lines(rule),
        "\n\n",
        preceded("your ticket:\n", list(i64)),
        "\n\n",
        preceded("nearby tickets:\n", lines(list(i64))),
    )
    .map(|(rules, yours, tix)| Input { rules, yours, tix })
    .read(s)
}

fn invalid_values(rules: &[(&str, i64, i64, i64, i64)], ticket: &[i64]) -> Vec<i64> {
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
    let Input { rules, yours: _, tix } = parse_rules(input);
    tix.iter().flat_map(|t| invalid_values(&rules, t)).sum()
}

pub fn part2(input: &str) -> i64 {
    let Input { rules, yours, tix } = parse_rules(input);
    let tix: Vec<Vec<i64>> =
        tix.into_iter().filter(|t| invalid_values(&rules, t).is_empty()).collect();
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
    let mut poss_set: Vec<HashSet<&str>> =
        poss.into_iter().map(|p| p.into_iter().map(|x| x.0).collect()).collect();
    while !poss_set.iter().all(|p| p.len() == 1) {
        let ones: HashSet<&str> =
            poss_set.iter().filter(|p| p.len() == 1).flatten().copied().collect();
        poss_set =
            poss_set.into_iter().map(|p| if p.len() == 1 { p } else { &p - &ones }).collect();
    }
    poss_set
        .into_iter()
        .map(|p| p.into_iter().next().unwrap())
        .zip(yours)
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| x.1)
        .product()
}
