use std::cell::Cell;
use std::collections::VecDeque;

use crate::year2017::day18::Instr::*;
use crate::year2017::day18::Val::*;

#[derive(Clone)]
enum Val {
    Lit(i64),
    Reg(usize),
}

#[derive(Clone)]
enum Instr {
    Snd(Val),
    Set(usize, Val),
    Add(usize, Val),
    Mul(usize, Val),
    Mod(usize, Val),
    Rcv(usize),
    Jgz(Val, Val),
}

#[derive(Clone)]
struct Sim {
    line: i64,
    reg: [i64; 26],
    instrs: Vec<Instr>,
}

fn reg(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

fn val(s: &str) -> Val {
    match s.parse::<i64>() {
        Ok(n) => Lit(n),
        Err(_) => Reg(reg(s)),
    }
}

fn parse_instrs(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["snd", v] => Snd(val(v)),
                ["set", r, v] => Set(reg(r), val(v)),
                ["add", r, v] => Add(reg(r), val(v)),
                ["mul", r, v] => Mul(reg(r), val(v)),
                ["mod", r, v] => Mod(reg(r), val(v)),
                ["rcv", r] => Rcv(reg(r)),
                ["jgz", a, b] => Jgz(val(a), val(b)),
                _ => panic!("Parse error: {}", line),
            },
        )
        .collect()
}

impl Sim {
    fn run<F1, F2>(&mut self, mut send: F1, mut recv: F2)
    where
        F1: FnMut(i64),
        F2: FnMut() -> Option<i64>,
    {
        while let Some(instr) = self.instrs.get(self.line as usize) {
            match instr {
                Snd(v) => send(self.val(v)),
                Set(r, v) => self.reg[*r] = self.val(v),
                Add(r, v) => self.reg[*r] += self.val(v),
                Mul(r, v) => self.reg[*r] *= self.val(v),
                Mod(r, v) => self.reg[*r] = self.reg[*r].rem_euclid(self.val(v)),
                Rcv(r) => match recv() {
                    Some(v) => self.reg[*r] = v,
                    None => break,
                },
                Jgz(a, b) => {
                    if self.val(a) > 0 {
                        self.line += self.val(b) - 1;
                    }
                }
            }
            self.line += 1;
        }
    }

    fn val(&self, v: &Val) -> i64 {
        match v {
            Lit(n) => *n,
            Reg(r) => self.reg[*r],
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut s = Sim {
        line: 0,
        reg: [0; 26],
        instrs: parse_instrs(input),
    };
    let v = Cell::new(0);
    s.run(|x| v.set(x), || (v.get() == 0).then(|| v.get()));
    v.get()
}

pub fn part2(input: &str) -> usize {
    let mut s0 = Sim {
        line: 0,
        reg: [0; 26],
        instrs: parse_instrs(input),
    };
    let mut s1 = s0.clone();
    s1.reg['p' as usize - 'a' as usize] = 1;
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    let mut p1_sends = 0;
    loop {
        s0.run(|x| q0.push_back(x), || q1.pop_front());
        s1.run(
            |x| {
                p1_sends += 1;
                q1.push_back(x)
            },
            || q0.pop_front(),
        );
        if q0.is_empty() && q1.is_empty() {
            return p1_sends;
        }
    }
}
