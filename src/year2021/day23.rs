use crate::utils::*;
use std::cmp::{max, min};

const DEST: [usize; 4] = [1, 3, 5, 7];

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Diagram {
    stacks: Vec<Vec<usize>>,
    lens: Vec<usize>,
    done: Vec<bool>,
}

fn parse(input: &str, p2: bool) -> Diagram {
    let mut stacks = vec![vec![]; 9];
    let mut lens = vec![2, 0, 1, 0, 1, 0, 1, 0, 2];
    let mut i = 0;
    for line in input.lines().rev() {
        for c in line.chars() {
            if "ABCD".contains(c) {
                stacks[DEST[i]].push((c as u8 - b'A') as usize + 1);
                i = (i + 1) % 4;
            }
        }
    }
    if p2 {
        stacks[1].splice(1..1, [4, 4]);
        stacks[3].splice(1..1, [2, 3]);
        stacks[5].splice(1..1, [1, 2]);
        stacks[7].splice(1..1, [3, 1]);
    }
    for i in DEST {
        lens[i] = stacks[i].len();
    }
    let done = vec![false; 4];
    Diagram { stacks, lens, done }
}

fn neighbors(diag: &Diagram) -> Vec<(usize, Diagram)> {
    let mut neighbs = vec![];
    for i in 0..diag.stacks.len() {
        if diag.stacks[i].is_empty() {
            continue;
        }
        if i % 2 == 0 {
            let c = diag.stacks[i].last().unwrap();
            let t = DEST[*c - 1];
            if diag.stacks[t].iter().all(|x| x == c)
                && (min(i, t) + 1..max(i, t))
                    .filter(|idx| idx % 2 == 0)
                    .all(|idx| diag.stacks[idx].is_empty())
            {
                let mut next = diag.clone();
                let mut cost = max(i, t) - min(i, t) + next.lens[i] - next.stacks[i].len()
                    + next.lens[t]
                    - next.stacks[t].len();
                cost *= 10_usize.pow(*c as u32 - 1);
                let v = next.stacks[i].pop().unwrap();
                if !next.stacks[i].is_empty() && next.stacks[i][0] == 0 {
                    next.stacks[i].pop();
                }
                next.stacks[t].push(v);
                if next.stacks[t].len() == next.lens[t] {
                    next.done[t / 2] = true;
                }
                neighbs.push((cost, next));
            }
        } else if !diag.done[i / 2] {
            let c = diag.stacks[i].last().unwrap();
            if i == DEST[*c - 1] && diag.stacks[i].iter().all(|x| x == c) {
                continue;
            }
            let t = DEST[*c - 1];
            let candidates = diag.stacks[t]
                .iter()
                .all(|x| x == c)
                .then(|| t)
                .into_iter()
                .chain((0..diag.stacks.len()).step_by(2));
            for j in candidates {
                if j != i
                    && diag.stacks[j].len() < diag.lens[j]
                    && (min(i, j) + 1..max(i, j))
                        .filter(|idx| idx % 2 == 0)
                        .all(|idx| diag.stacks[idx].is_empty())
                {
                    let mut next = diag.clone();
                    let mut cost = max(i, j) - min(i, j) + next.lens[i] - next.stacks[i].len()
                        + next.lens[j]
                        - next.stacks[j].len()
                        + (j == t) as usize;
                    cost *= 10_usize.pow(*c as u32 - 1);
                    let v = next.stacks[i].pop().unwrap();
                    next.stacks[j].push(v);
                    if j == t && next.stacks[j].len() == next.lens[j] {
                        next.done[j / 2] = true;
                    }
                    if j != t && diag.stacks[j].len() < diag.lens[j] - 1 {
                        let mut next2 = next.clone();
                        next2.stacks[j].insert(0, 0);
                        neighbs.push((cost - 10_usize.pow(*c as u32 - 1), next2));
                    }
                    neighbs.push((cost, next));
                }
            }
        }
    }
    neighbs
}

pub fn part1(input: &str) -> Option<usize> {
    dijkstra(parse(input, false), neighbors)
        .find(|x| x.1.done.iter().all(|x| *x))
        .map(|x| x.0)
}

pub fn part2(input: &str) -> Option<usize> {
    dijkstra(parse(input, true), neighbors)
        .find(|x| x.1.done.iter().all(|x| *x))
        .map(|x| x.0)
}
