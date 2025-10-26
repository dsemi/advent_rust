use crate::utils::UniqueIdx;
use crate::utils::parsers::*;
use hashbrown::HashSet;
use num::integer::lcm;
use std::cmp::max;

fn network(
    input: &str,
    is_start: fn(&str) -> bool,
    is_end: fn(&str) -> bool,
) -> (Vec<usize>, HashSet<usize>, Vec<usize>, Vec<usize>) {
    let mut starts = Vec::new();
    let mut ends = HashSet::new();
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    let mut ui = UniqueIdx::new();
    for line in input.lines() {
        let (src, (left, right)) =
            separated_pair(alphanumeric1, " = ", delimited('(', coord(alphanumeric1), ')'))
                .read(line);
        let k = ui.idx(src);
        if is_start(src) {
            starts.push(k);
        }
        if is_end(src) {
            ends.insert(k);
        }
        lefts.resize(max(lefts.len(), k + 1), 0);
        lefts[k] = ui.idx(left);
        rights.resize(max(lefts.len(), k + 1), 0);
        rights[k] = ui.idx(right);
    }
    (starts, ends, lefts, rights)
}

pub fn solve(input: &str, is_start: fn(&str) -> bool, is_end: fn(&str) -> bool) -> usize {
    let (dirs, net) = separated_pair(alpha1, "\n\n", winnow::token::rest).read(input);
    let (starts, ends, lefts, rights) = network(net, is_start, is_end);
    starts
        .into_iter()
        .map(|node| {
            dirs.chars()
                .cycle()
                .enumerate()
                .try_fold(node, |node, (i, d)| {
                    if ends.contains(&node) {
                        Err(i)
                    } else {
                        Ok(if d == 'L' { lefts[node] } else { rights[node] })
                    }
                })
                .unwrap_err()
        })
        .reduce(lcm)
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    solve(input, |s| s == "AAA", |e| e == "ZZZ")
}

pub fn part2(input: &str) -> usize {
    solve(input, |s| s.ends_with('A'), |e| e.ends_with('Z'))
}
