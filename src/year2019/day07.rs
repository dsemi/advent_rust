use crate::year2019::intcode;
use itertools::Itertools;
use std::collections::VecDeque;

fn chain(p: &intcode::Program, phases: Vec<i64>, cycle: bool) -> Vec<i64> {
    let mut progs = Vec::new();
    for phase in phases.iter() {
        let mut prog = p.clone();
        prog.input.push_back(*phase);
        progs.push(prog);
    }
    let last = progs.len() - 1;
    progs[0].input.push_back(0);
    let mut result = Vec::new();
    loop {
        for i in 0..progs.len() {
            if progs[i].done {
                return result;
            }
            progs[i].run();
            if i < last {
                let mut vs = progs[i].output.drain(..).collect();
                progs[i + 1].input.append(&mut vs);
            }
        }
        let mut vs: VecDeque<i64> = progs[last].output.drain(..).collect();
        result.extend(vs.iter().copied());
        if cycle {
            progs[0].input.append(&mut vs);
        }
    }
}

pub fn part1(input: &str) -> Option<i64> {
    let prog = intcode::new(input);
    (0..5)
        .permutations(5)
        .map(|perm| chain(&prog, perm, false)[0])
        .max()
}

pub fn part2(input: &str) -> Option<i64> {
    let prog = intcode::new(input);
    (5..10)
        .permutations(5)
        .map(|perm| {
            let ns = chain(&prog, perm, true);
            ns[ns.len() - 1]
        })
        .max()
}
