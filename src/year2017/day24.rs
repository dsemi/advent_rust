use crate::utils::parsers::*;
use ahash::AHashSet;
use std::cmp::max;

#[derive(Clone, Copy)]
struct Pipe {
    id: usize,
    a: u32,
    b: u32,
}

fn parse_pipes(input: &str) -> Vec<Pipe> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (a, b) = separated_pair(u32, tag("/"), u32)(line).unwrap().1;
            Pipe { id, a, b }
        })
        .collect()
}

fn build<T: Copy + Ord>(
    f: fn(T, Pipe) -> T,
    neighbs: &[Vec<Pipe>],
    visited: &mut AHashSet<u64>,
    used: u64,
    port: u32,
    curr: T,
) -> T {
    if !visited.insert(used) {
        return curr;
    }
    neighbs[port as usize]
        .iter()
        .filter(|pipe| used & (1 << pipe.id) == 0)
        .map(|&p| {
            let used = used | (1 << p.id);
            build(f, neighbs, visited, used, p.a + p.b - port, f(curr, p))
        })
        .fold(curr, max)
}

fn solve<T: Copy + Ord>(input: &str, start: T, step: fn(T, Pipe) -> T) -> T {
    let pipes = parse_pipes(input);
    let mx = pipes.iter().flat_map(|p| [p.a, p.b]).max().unwrap() as usize;
    let mut neighbs = vec![vec![]; mx + 1];
    for pipe in pipes {
        neighbs[pipe.a as usize].push(pipe);
        if pipe.a != pipe.b {
            neighbs[pipe.b as usize].push(pipe);
        }
    }
    build(step, &neighbs, &mut AHashSet::new(), 0, 0, start)
}

pub fn part1(input: &str) -> u32 {
    solve(input, 0, |s, p| s + p.a + p.b)
}

pub fn part2(input: &str) -> u32 {
    solve(input, (0, 0), |s, p| (s.0 + 1, s.1 + p.a + p.b)).1
}
