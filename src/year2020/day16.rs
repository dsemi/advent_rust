use std::collections::HashSet;

fn parse_rules(s: &str) -> (Vec<(String, i64, i64, i64, i64)>, Vec<i64>, Vec<Vec<i64>>) {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let rules: Vec<(String, i64, i64, i64, i64)> = parts[0]
        .lines()
        .map(|line| scan_fmt!(line, "{[^:]}: {}-{} or {}-{}", String, i64, i64, i64, i64).unwrap())
        .collect();
    let yours: Vec<i64> = parts[1]
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let others: Vec<Vec<i64>> = parts[2]
        .lines()
        .skip(1)
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, yours, others)
}

fn invalid_values(rules: &Vec<(String, i64, i64, i64, i64)>, ticket: &Vec<i64>) -> Vec<i64> {
    ticket
        .iter()
        .filter(|&field| {
            !rules
                .iter()
                .any(|(_, a, b, c, d)| a <= field && field <= b || c <= field && field <= d)
        })
        .map(|x| *x)
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let (rules, _, tix) = parse_rules(input);
    tix.iter().flat_map(|t| invalid_values(&rules, t)).sum()
}

pub fn part2(input: &str) -> i64 {
    let (rules, yours, tix) = parse_rules(input);
    let tix: Vec<Vec<i64>> = tix
        .into_iter()
        .filter(|t| invalid_values(&rules, &t).is_empty())
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
    let mut poss_set: Vec<HashSet<String>> = poss
        .into_iter()
        .map(|p| p.into_iter().map(|x| x.0).collect())
        .collect();
    while !poss_set.iter().all(|p| p.len() == 1) {
        let ones: HashSet<String> = poss_set
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
    let fields: Vec<String> = poss_set
        .into_iter()
        .map(|p| p.into_iter().next().unwrap())
        .collect();
    fields
        .into_iter()
        .zip(yours)
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| x.1)
        .product()
}
