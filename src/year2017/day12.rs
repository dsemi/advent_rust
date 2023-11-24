use crate::utils::parsers::*;
use crate::utils::*;

fn parse_pipes(input: &str) -> Vec<Vec<usize>> {
    let pipe = |i| preceded(pair(digit1, tag(" <-> ")), list(usize))(i);
    input.lines().map(|line| pipe(line).unwrap().1).collect()
}

pub fn part1(input: &str) -> usize {
    let m = parse_pipes(input);
    bfs(0, |&n| m[n].iter().copied()).count()
}

pub fn part2(input: &str) -> usize {
    let m = parse_pipes(input);
    let mut seen = vec![false; m.len()];
    (0..m.len())
        .filter_map(|n| {
            (!seen[n]).then(|| bfs(n, |&x| m[x].iter().copied()).for_each(|(_, x)| seen[x] = true))
        })
        .count()
}
