use std::collections::HashMap;
use std::collections::HashSet;

use crate::year2018::day16::Op::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

static OPS: [Op; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

fn eval(v: &mut Vec<i64>, op: Op, a: i64, b: i64, c: i64) {
    v[c as usize] = match op {
        Addr => v[a as usize] + v[b as usize],
        Addi => v[a as usize] + b,
        Mulr => v[a as usize] * v[b as usize],
        Muli => v[a as usize] * b,
        Banr => v[a as usize] & v[b as usize],
        Bani => v[a as usize] & b,
        Borr => v[a as usize] | v[b as usize],
        Bori => v[a as usize] | b,
        Setr => v[a as usize],
        Seti => a,
        Gtir => (a > v[b as usize]) as i64,
        Gtri => (v[a as usize] > b) as i64,
        Gtrr => (v[a as usize] > v[b as usize]) as i64,
        Eqir => (a == v[b as usize]) as i64,
        Eqri => (v[a as usize] == b) as i64,
        Eqrr => (v[a as usize] == v[b as usize]) as i64,
    }
}

fn test_sample(sample: &str) -> (i64, HashSet<Op>) {
    let pts = sample.lines().collect::<Vec<_>>();
    let (before, instr, after) = (pts[0], pts[1], pts[2]);
    let ns = instr
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let (op, a, b, c) = (ns[0], ns[1], ns[2], ns[3]);
    let mem1 = before.split(&['[', ']'][..]).collect::<Vec<_>>()[1]
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let mem2: Vec<i64> = after.split(&['[', ']'][..]).collect::<Vec<_>>()[1]
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut result = (op, HashSet::new());
    for cmd in OPS {
        let mut mem = mem1.clone();
        eval(&mut mem, cmd, a, b, c);
        if mem == mem2 {
            result.1.insert(cmd);
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    input
        .rsplit("\n\n")
        .skip(2)
        .filter(|&sample| test_sample(sample).1.len() >= 3)
        .count()
}

fn determine_op_codes(mut m: HashMap<i64, HashSet<Op>>) -> HashMap<i64, Op> {
    while m.values().any(|v| v.len() != 1) {
        for poss in m
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|v| v.len() == 1)
        {
            for p in poss {
                for v in m.values_mut() {
                    if v.len() != 1 {
                        v.remove(&p);
                    }
                }
            }
        }
    }
    m.into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect()
}

pub fn part2(input: &str) -> i64 {
    let pts = input.rsplit("\n\n").collect::<Vec<_>>();
    let prog = pts[0];
    let samples = &pts[2..];
    let mut m = HashMap::new();
    for sample in samples {
        let (k, v) = test_sample(sample);
        let e = m.entry(k).or_insert(HashSet::new());
        e.extend(v);
    }
    let ops = determine_op_codes(m);
    let mut mem = vec![0; 4];
    for line in prog.lines() {
        let pts = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        eval(&mut mem, ops[&pts[0]], pts[1], pts[2], pts[3]);
    }
    mem[0]
}
