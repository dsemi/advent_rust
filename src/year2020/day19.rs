use crate::utils::parsers2::*;
use itertools::Itertools;
use Rule::*;

enum Rule {
    Single(u8),
    Multi(Vec<Vec<usize>>),
}

fn parse_rules(s: &str) -> (Vec<Rule>, Vec<&str>) {
    let (rules, messages) = s.split_once("\n\n").unwrap();
    (
        rules
            .lines()
            .map(|line| {
                let (idx, content) = line.split_once(": ").unwrap();
                (
                    idx.usize(),
                    if content.starts_with('"') {
                        Single(content.as_bytes()[1])
                    } else {
                        Multi(
                            content
                                .split(" | ")
                                .map(|part| part.split(' ').map(usize::read).collect())
                                .collect(),
                        )
                    },
                )
            })
            .sorted_unstable_by_key(|x| x.0)
            .map(|x| x.1)
            .collect(),
        messages.lines().collect(),
    )
}

fn count_matches(rules: Vec<Rule>, messages: Vec<&str>) -> usize {
    fn check(rules: &[Rule], s: &[u8], seq: &[usize]) -> bool {
        if s.is_empty() || seq.is_empty() {
            return s.is_empty() && seq.is_empty();
        }
        match &rules[seq[0]] {
            Single(c) => &s[0] == c && check(rules, &s[1..], &seq[1..]),
            Multi(rss) => rss
                .iter()
                .any(|rs| check(rules, s, &[rs, &seq[1..]].concat())),
        }
    }

    messages
        .into_iter()
        .filter(|&message| check(&rules, message.as_bytes(), &[0]))
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
