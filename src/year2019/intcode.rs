use crate::utils::parsers::*;
use Instr::*;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Program {
    idx: i64,
    rel_base: i64,
    mem: Vec<i64>,
    pub done: bool,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
}

enum Instr {
    Add(i64, i64, i64),
    Mul(i64, i64, i64),
    Sav(i64),
    Out(i64),
    Jit(i64, i64),
    Jif(i64, i64),
    Lt(i64, i64, i64),
    Eql(i64, i64, i64),
    Arb(i64),
    Hlt,
}

pub fn new(input: &str) -> Program {
    Program {
        idx: 0,
        rel_base: 0,
        mem: list(i64).read(input),
        done: false,
        input: VecDeque::new(),
        output: VecDeque::new(),
    }
}

impl Index<i64> for Program {
    type Output = i64;

    fn index(&self, i: i64) -> &Self::Output {
        self.mem.get(i as usize).unwrap_or(&0)
    }
}

impl IndexMut<i64> for Program {
    fn index_mut(&mut self, i: i64) -> &mut Self::Output {
        let idx = i as usize;
        if idx >= self.mem.len() {
            self.mem.resize(idx + 1, 0);
        }
        &mut self.mem[idx]
    }
}

impl Program {
    fn arg(&mut self, idx: i64) -> i64 {
        let mode = self[idx] / 10_i64.pow((self.idx - idx + 1).try_into().unwrap()) % 10;
        let val = match mode {
            0 => self[self.idx],
            1 => self.idx,
            2 => self[self.idx] + self.rel_base,
            _ => panic!("Unknown mode"),
        };
        self.idx += 1;
        val
    }

    fn parse_instr(&mut self) -> Instr {
        let idx = self.idx;
        let op_code = self[idx] % 100;
        self.idx += 1;
        match op_code {
            1 => Add(self.arg(idx), self.arg(idx), self.arg(idx)),
            2 => Mul(self.arg(idx), self.arg(idx), self.arg(idx)),
            3 => Sav(self.arg(idx)),
            4 => Out(self.arg(idx)),
            5 => Jit(self.arg(idx), self.arg(idx)),
            6 => Jif(self.arg(idx), self.arg(idx)),
            7 => Lt(self.arg(idx), self.arg(idx), self.arg(idx)),
            8 => Eql(self.arg(idx), self.arg(idx), self.arg(idx)),
            9 => Arb(self.arg(idx)),
            99 => Hlt,
            _ => panic!("Unknown instr {}", op_code),
        }
    }

    pub fn recv<const N: usize>(&mut self) -> Option<[i64; N]> {
        (self.output.len() >= N).then(|| {
            let mut result = [0; N];
            for r in result.iter_mut() {
                *r = self.output.pop_front().unwrap();
            }
            result
        })
    }

    pub fn run(&mut self) {
        assert!(!self.done);
        loop {
            match self.parse_instr() {
                Add(a, b, c) => self[c] = self[a] + self[b],
                Mul(a, b, c) => self[c] = self[a] * self[b],
                Sav(a) => {
                    if let Some(v) = self.input.pop_front() {
                        self[a] = v;
                    } else {
                        self.idx -= 2;
                        break;
                    }
                }
                Out(a) => self.output.push_back(self[a]),
                Jit(a, b) => self.idx = if self[a] != 0 { self[b] } else { self.idx },
                Jif(a, b) => self.idx = if self[a] == 0 { self[b] } else { self.idx },
                Lt(a, b, c) => self[c] = (self[a] < self[b]) as i64,
                Eql(a, b, c) => self[c] = (self[a] == self[b]) as i64,
                Arb(a) => self.rel_base += self[a],
                Hlt => {
                    self.done = true;
                    break;
                }
            }
        }
    }

    pub fn run_no_io(mut self, a: i64, b: i64) -> i64 {
        self[1] = a;
        self[2] = b;
        self.run();
        assert!(self.done);
        self[0]
    }

    pub fn run_with_input(mut self, inp: &[i64]) -> impl Iterator<Item = i64> {
        self.input.extend(inp);
        self.run();
        assert!(self.done);
        self.output.into_iter()
    }
}
