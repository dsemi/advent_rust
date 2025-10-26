use crate::utils::parsers::*;
use Instr::*;
use Val::*;
use advent::Parser;
use std::cell::Cell;
use std::collections::VecDeque;

#[derive(Clone, Parser)]
#[parser(dont_parse_name)]
enum Val {
    Lit(i64),
    Reg(#[parser(impl = reg)] usize),
}

fn reg(i: &mut &str) -> ModalResult<usize> {
    any.map(|c| c as usize - 'a' as usize).parse_next(i)
}

#[derive(Clone, Parser)]
enum Instr {
    Snd(Val),
    Set(#[parser(impl = reg)] usize, Val),
    Add(#[parser(impl = reg)] usize, Val),
    Mul(#[parser(impl = reg)] usize, Val),
    Mod(#[parser(impl = reg)] usize, Val),
    Rcv(#[parser(impl = reg)] usize),
    Jgz(Val, Val),
}

#[derive(Clone)]
struct Sim {
    line: i64,
    reg: [i64; 26],
    instrs: Vec<Instr>,
    sends: usize,
}

impl Sim {
    fn parse(input: &str) -> Self {
        Self { line: 0, reg: [0; 26], instrs: lines(instr).read(input), sends: 0 }
    }

    fn run<F1, F2>(&mut self, mut send: F1, mut recv: F2)
    where
        F1: FnMut(i64),
        F2: FnMut() -> Option<i64>,
    {
        while let Some(instr) = self.instrs.get(self.line as usize) {
            match instr {
                Snd(v) => {
                    self.sends += 1;
                    send(self.val(v))
                }
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
    let mut s = Sim::parse(input);
    let v = Cell::new(0);
    s.run(|x| v.set(x), || (v.get() == 0).then(|| v.get()));
    v.get()
}

pub fn part2(input: &str) -> usize {
    let mut s0 = Sim::parse(input);
    let mut s1 = s0.clone();
    s1.reg['p' as usize - 'a' as usize] = 1;
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    loop {
        s0.run(|x| q0.push_back(x), || q1.pop_front());
        s1.run(|x| q1.push_back(x), || q0.pop_front());
        if q0.is_empty() && q1.is_empty() {
            return s1.sends;
        }
    }
}
