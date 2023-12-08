use crate::utils::parsers::*;
use ahash::{AHashMap, AHashSet};

fn bag<'a>(i: &mut &'a str) -> PResult<&'a str> {
    terminated(
        (alpha1, space1, alpha1).recognize(),
        (alt((" bags", " bag")), opt('.')),
    )
    .parse_next(i)
}

fn bags<'a>(i: &mut &'a str) -> PResult<Vec<(u32, &'a str)>> {
    let res = list(separated_pair(u32, space1, bag))
        .parse_next(i)
        .unwrap_or_default();
    rest.parse_next(i)?;
    Ok(res)
}

fn parse_bags(input: &str) -> AHashMap<&str, Vec<(u32, &str)>> {
    lines_iter(input, separated_pair(bag, " contain ", bags)).collect()
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
