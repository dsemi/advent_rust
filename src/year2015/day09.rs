use crate::utils::{held_karp, UniqueIdx};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u64};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::{max, min};

fn parse(i: &str) -> IResult<&str, (&str, &str, usize)> {
    map(
        tuple((alpha1, tag(" to "), alpha1, tag(" = "), u64)),
        |(k1, _, k2, _, v)| (k1, k2, v as usize),
    )(i)
}

fn all_path_distances(input: &str, f: fn(usize, usize) -> usize) -> Option<usize> {
    let mut adj = Vec::new();
    let mut ui = UniqueIdx::new();
    let fake = ui.idx("fake");
    for line in input.lines() {
        let (k1, k2, v) = parse(line).unwrap().1;
        let n1 = ui.idx(k1);
        let n2 = ui.idx(k2);
        while max(n1, n2) >= adj.len() {
            adj.push(Vec::new());
        }
        while n1 >= adj[n2].len() {
            adj[n2].push(0);
        }
        while n2 >= adj[n1].len() {
            adj[n1].push(0);
        }
        adj[n1][n2] = v;
        adj[n2][n1] = v;
    }
    let len = adj.len();
    adj[fake].extend(vec![0; len]);
    held_karp(&adj, f)
}

pub fn part1(input: &str) -> Option<usize> {
    all_path_distances(input, min)
}

pub fn part2(input: &str) -> Option<usize> {
    all_path_distances(input, max)
}
