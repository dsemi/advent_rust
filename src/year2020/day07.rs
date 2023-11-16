use crate::utils::parsers::*;
use ahash::{AHashMap, AHashSet};

fn bag(i: &str) -> IResult<&str, &str> {
    terminated(
        recognize(tuple((alpha1, space1, alpha1))),
        alt((tag(" bags"), tag(" bag"))),
    )(i)
}

fn parse_bags(s: &str) -> AHashMap<&str, Vec<(u32, &str)>> {
    s.lines()
        .map(|line| {
            separated_pair(
                bag,
                tag(" contain "),
                list(separated_pair(u32, space1, bag)),
            )(line)
            .unwrap()
            .1
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let m = parse_bags(input);
    let mut rev = AHashMap::new();
    for (k, v) in m {
        for (_, k2) in v {
            rev.entry(k2).or_insert_with(Vec::new).push(k);
        }
    }
    let mut stack = rev[&"shiny gold"].clone();
    let mut visited = AHashSet::new();
    let mut ans = 0;
    while let Some(v) = stack.pop() {
        if visited.insert(v) {
            ans += 1;
            if let Some(vs) = rev.get(v) {
                stack.extend(vs);
            }
        }
    }
    ans
}

fn count_bags(m: &AHashMap<&str, Vec<(u32, &str)>>, k: &str) -> u32 {
    m[k].iter().map(|(n, k2)| n + n * count_bags(m, k2)).sum()
}

pub fn part2(input: &str) -> u32 {
    count_bags(&parse_bags(input), "shiny gold")
}
