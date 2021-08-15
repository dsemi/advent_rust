use crate::year2020::day19::Rule::Multi;
use crate::year2020::day19::Rule::Single;

#[derive(Debug)]
enum Rule {
    Single(u8),
    Multi(Vec<Vec<usize>>),
}

fn parse_rules(s: &str) -> (Vec<Rule>, Vec<&str>) {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let mut rules: Vec<(usize, Rule)> = Vec::new();
    for line in parts[0].lines() {
        let ps: Vec<&str> = line.split(": ").collect();
        let v = if ps[1].starts_with('"') {
            Single(ps[1].chars().nth(1).unwrap() as u8)
        } else {
            Multi(
                ps[1]
                    .split(" | ")
                    .map(|part| part.split(' ').map(|x| x.parse().unwrap()).collect())
                    .collect(),
            )
        };
        rules.push((ps[0].parse().unwrap(), v));
    }
    rules.sort_unstable_by_key(|x| x.0);
    for (i, v) in rules.iter().enumerate() {
        assert!(i == v.0);
    }
    (
        rules.into_iter().map(|x| x.1).collect(),
        parts[1].lines().collect(),
    )
}

fn count_matches(rules: Vec<Rule>, messages: Vec<&str>) -> usize {
    fn go<'a>(rules: &'a [Rule], rule: &Rule, s: &'a [u8]) -> Vec<&'a [u8]> {
        if s.is_empty() {
            return vec![];
        }
        match rule {
            Single(c) => {
                if s[0] == *c {
                    vec![&s[1..]]
                } else {
                    vec![]
                }
            }
            // Need vec b/c option could partially succeed down the wrong path and not
            // backtrack properly.
            Multi(rss) => rss
                .iter()
                .flat_map(|rs| {
                    rs.iter().fold(vec![s], |acc, r| {
                        acc.into_iter()
                            .flat_map(|x| go(rules, &rules[*r], x))
                            .collect()
                    })
                })
                .collect(),
        }
    }
    messages
        .into_iter()
        .filter(|&message| {
            go(&rules, &rules[0], message.as_bytes())
                .into_iter()
                .any(|x| x.is_empty())
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    let (rules, messages) = parse_rules(input);
    count_matches(rules, messages)
}

pub fn part2(input: &str) -> usize {
    let (mut rules, messages) = parse_rules(input);
    rules[8] = Multi(vec![vec![42], vec![42, 8]]);
    rules[11] = Multi(vec![vec![42, 31], vec![42, 11, 31]]);
    count_matches(rules, messages)
}
