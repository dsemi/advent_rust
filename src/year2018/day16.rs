use crate::year2018::day16::Op::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, Eq, FromPrimitive, Hash, PartialEq)]
enum Op {
    Addr = 1,
    Addi = 2,
    Mulr = 4,
    Muli = 8,
    Banr = 16,
    Bani = 32,
    Borr = 64,
    Bori = 128,
    Setr = 256,
    Seti = 512,
    Gtir = 1024,
    Gtri = 2048,
    Gtrr = 4096,
    Eqir = 8192,
    Eqri = 16384,
    Eqrr = 32768,
}

static OPS: [Op; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

fn eval(v: &mut Vec<usize>, op: Op, a: usize, b: usize, c: usize) {
    v[c as usize] = match op {
        Addr => v[a] + v[b],
        Addi => v[a] + b,
        Mulr => v[a] * v[b],
        Muli => v[a] * b,
        Banr => v[a] & v[b],
        Bani => v[a] & b,
        Borr => v[a] | v[b],
        Bori => v[a] | b,
        Setr => v[a],
        Seti => a,
        Gtir => (a > v[b]) as usize,
        Gtri => (v[a] > b) as usize,
        Gtrr => (v[a] > v[b]) as usize,
        Eqir => (a == v[b]) as usize,
        Eqri => (v[a] == b) as usize,
        Eqrr => (v[a] == v[b]) as usize,
    }
}

fn test_sample(sample: &str) -> (usize, u16) {
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
    let mem2: Vec<usize> = after.split(&['[', ']'][..]).collect::<Vec<_>>()[1]
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut result = (op as usize, 0);
    for cmd in OPS {
        let mut mem = mem1.clone();
        eval(&mut mem, cmd, a, b, c);
        if mem == mem2 {
            result.1 |= cmd as u16;
        }
    }
    result
}

pub fn part1(input: &str) -> usize {
    input
        .rsplit("\n\n")
        .skip(2)
        .filter(|&sample| test_sample(sample).1.count_ones() >= 3)
        .count()
}

fn determine_op_codes(mut m: Vec<u16>) -> Vec<Op> {
    while m.iter().any(|v| v.count_ones() != 1) {
        let uniques = m
            .iter()
            .filter(|v| v.count_ones() == 1)
            .copied()
            .collect::<Vec<_>>();
        for p in uniques {
            for v in m.iter_mut() {
                if *v != p {
                    *v &= !p;
                }
            }
        }
    }
    m.into_iter()
        .map(|v| FromPrimitive::from_u16(v).unwrap())
        .collect()
}

pub fn part2(input: &str) -> usize {
    let pts = input.rsplit("\n\n").collect::<Vec<_>>();
    let prog = pts[0];
    let samples = &pts[2..];
    let mut m = Vec::new();
    for sample in samples {
        let (k, v) = test_sample(sample);
        if k >= m.len() {
            m.resize(k + 1, 0);
        }
        m[k] |= v;
    }
    let ops = determine_op_codes(m);
    let mut mem = vec![0; 4];
    for line in prog.lines() {
        let pts = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        eval(&mut mem, ops[pts[0]], pts[1], pts[2], pts[3]);
    }
    mem[0]
}
