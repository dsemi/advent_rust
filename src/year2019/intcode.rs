use std::collections::VecDeque;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

fn parse_instrs(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

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
        mem: parse_instrs(input),
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
    fn arg(&self, i: i64) -> i64 {
        let mode = self[self.idx] / 10_i64.pow((i + 1).try_into().unwrap()) % 10;
        match mode {
            0 => self[self.idx + i],
            1 => self.idx + i,
            2 => self[self.idx + i] + self.rel_base,
            _ => panic!("Unknown mode"),
        }
    }

    fn parse_instr(&mut self) -> Instr {
        let op_code = self[self.idx] % 100;
        match op_code {
            1 => Instr::Add(self.arg(1), self.arg(2), self.arg(3)),
            2 => Instr::Mul(self.arg(1), self.arg(2), self.arg(3)),
            3 => Instr::Sav(self.arg(1)),
            4 => Instr::Out(self.arg(1)),
            5 => Instr::Jit(self.arg(1), self.arg(2)),
            6 => Instr::Jif(self.arg(1), self.arg(2)),
            7 => Instr::Lt(self.arg(1), self.arg(2), self.arg(3)),
            8 => Instr::Eql(self.arg(1), self.arg(2), self.arg(3)),
            9 => Instr::Arb(self.arg(1)),
            99 => Instr::Hlt,
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
                Instr::Add(a, b, c) => {
                    self[c] = self[a] + self[b];
                    self.idx += 4;
                }
                Instr::Mul(a, b, c) => {
                    self[c] = self[a] * self[b];
                    self.idx += 4;
                }
                Instr::Sav(a) => {
                    if self.input.is_empty() {
                        break;
                    }
                    self[a] = self.input.pop_front().unwrap();
                    self.idx += 2;
                }
                Instr::Out(a) => {
                    self.output.push_back(self[a]);
                    self.idx += 2;
                }
                Instr::Jit(a, b) => {
                    if self[a] != 0 {
                        self.idx = self[b];
                    } else {
                        self.idx += 3;
                    }
                }
                Instr::Jif(a, b) => {
                    if self[a] == 0 {
                        self.idx = self[b];
                    } else {
                        self.idx += 3;
                    }
                }
                Instr::Lt(a, b, c) => {
                    self[c] = (self[a] < self[b]) as i64;
                    self.idx += 4;
                }
                Instr::Eql(a, b, c) => {
                    self[c] = (self[a] == self[b]) as i64;
                    self.idx += 4;
                }
                Instr::Arb(a) => {
                    self.rel_base += self[a];
                    self.idx += 2;
                }
                Instr::Hlt => {
                    self.done = true;
                    break;
                }
            }
        }
    }
}

pub fn run_no_io(a: i64, b: i64, mut prog: Program) -> i64 {
    prog[1] = a;
    prog[2] = b;
    prog.run();
    assert!(prog.done);
    prog[0]
}

pub fn run_with_input(inp: Vec<i64>, mut prog: Program) -> Vec<i64> {
    for v in inp.into_iter() {
        prog.input.push_back(v);
    }
    prog.run();
    assert!(prog.done);
    prog.output.iter().copied().collect()
}
