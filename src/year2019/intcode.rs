use std::collections::HashMap;
use std::convert::TryInto;

pub fn parse_instrs(input: &str) -> HashMap<i64, i64> {
    input
        .split(",")
        .enumerate()
        .map(|(i, x)| (i as i64, x.parse().unwrap()))
        .collect()
}

#[derive(Clone)]
struct Program {
    idx: i64,
    rel_base: i64,
    mem: HashMap<i64, i64>,
}

enum Action {
    Input,
    Output(i64),
    Halt(HashMap<i64, i64>),
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

impl Program {
    fn val(&self, i: i64) -> i64 {
        *self.mem.get(&i).unwrap_or(&0)
    }

    fn arg(&self, i: i64) -> i64 {
        let mode =
            *self.mem.get(&self.idx).unwrap_or(&0) / 10_i64.pow((i + 1).try_into().unwrap()) % 10;
        match mode {
            0 => self.val(self.idx + i),
            1 => self.idx + i,
            2 => self.val(self.idx + i) + self.rel_base,
            _ => panic!("Unknown mode"),
        }
    }

    #[allow(dead_code)]
    fn arg2(&self, i: i64, mode: i64) -> i64 {
        match mode {
            0 => self.val(self.idx + i),
            1 => self.idx + i,
            2 => self.val(self.idx + i) + self.rel_base,
            _ => panic!("Unknown mode"),
        }
    }

    fn input(&mut self, v: i64) {
        self.mem.insert(self.arg(1), v);
        self.idx += 2;
    }

    fn output(&mut self) {
        self.idx += 2;
    }

    fn parse_instr(&mut self) -> Instr {
        let op_code = self.val(self.idx) % 100;
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

    fn run(&mut self) -> Action {
        loop {
            match self.parse_instr() {
                Instr::Add(a, b, c) => {
                    self.mem.insert(c, self.val(a) + self.val(b));
                    self.idx += 4;
                }
                Instr::Mul(a, b, c) => {
                    self.mem.insert(c, self.val(a) * self.val(b));
                    self.idx += 4;
                }
                Instr::Sav(_a) => {
                    return Action::Input;
                }
                Instr::Out(a) => {
                    return Action::Output(self.val(a));
                }
                Instr::Jit(a, b) => {
                    if self.val(a) != 0 {
                        self.idx = self.val(b);
                    } else {
                        self.idx += 3;
                    }
                }
                Instr::Jif(a, b) => {
                    if self.val(a) == 0 {
                        self.idx = self.val(b);
                    } else {
                        self.idx += 3;
                    }
                }
                Instr::Lt(a, b, c) => {
                    self.mem.insert(c, (self.val(a) < self.val(b)) as i64);
                    self.idx += 4;
                }
                Instr::Eql(a, b, c) => {
                    self.mem.insert(c, (self.val(a) == self.val(b)) as i64);
                    self.idx += 4;
                }
                Instr::Arb(a) => {
                    self.rel_base += self.val(a);
                    self.idx += 2;
                }
                Instr::Hlt => return Action::Halt(self.mem.clone()),
            }
        }
    }
}

pub fn run_no_io(a: i64, b: i64, mut mem: HashMap<i64, i64>) -> i64 {
    mem.insert(1, a);
    mem.insert(2, b);
    let mut prog = Program {
        idx: 0,
        rel_base: 0,
        mem: mem,
    };
    match prog.run() {
        Action::Halt(mem) => *mem.get(&0).unwrap(),
        _ => panic!("No IO"),
    }
}

pub fn run_with_input(inp: Vec<i64>, mem: HashMap<i64, i64>) -> Vec<i64> {
    let mut out = Vec::new();
    let mut prog = Program {
        idx: 0,
        rel_base: 0,
        mem: mem,
    };
    loop {
        match prog.run() {
            Action::Input => {
                prog.input(inp[0]);
            }
            Action::Output(v) => {
                out.push(v);
                prog.output();
            }
            Action::Halt(_) => break,
        }
    }
    out
}
