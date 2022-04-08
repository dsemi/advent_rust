use Instr::*;
use Value::*;

#[derive(Clone, Copy)]
enum Value {
    Reg(usize),
    Lit(i64),
}

#[derive(Clone)]
enum Instr {
    Cpy(Value, Value),
    Inc(usize),
    Dec(usize),
    Tgl(usize),
    Out(Value),
    Jnz(Value, Value),
    Add(usize, usize),
    Mul(Value, usize, usize, usize),
    Nop,
}

#[derive(Clone)]
pub struct Sim {
    pub regs: [i64; 4],
    line: i64,
    instrs: Vec<Instr>,
}

fn optimize(instrs: &mut [Instr]) {
    for i in 0..instrs.len() {
        if i + 6 <= instrs.len() {
            match instrs[i..i + 6] {
                [Cpy(a, Reg(d)), Inc(c), Dec(d2), Jnz(Reg(d3), Lit(-2)), Dec(b), Jnz(Reg(b2), Lit(-5))]
                    if d == d2 && d == d3 && b == b2 =>
                {
                    instrs[i] = Mul(a, b, c, d);
                    instrs[i + 1] = Nop;
                    instrs[i + 2] = Nop;
                    instrs[i + 3] = Nop;
                    instrs[i + 4] = Nop;
                    instrs[i + 5] = Nop;
                    continue;
                }
                _ => (),
            }
        }
        if i + 3 <= instrs.len() {
            match instrs[i..i + 3] {
                [Inc(a), Dec(b), Jnz(Reg(b2), Lit(-2))] if b == b2 => {
                    instrs[i] = Add(a, b);
                    instrs[i + 1] = Nop;
                    instrs[i + 2] = Nop;
                    continue;
                }
                _ => (),
            }
        }
    }
}

pub fn parse_instrs(input: &str) -> Sim {
    fn value(x: &str) -> Value {
        match x.chars().next().unwrap() {
            c @ 'a'..='d' => Reg(c as usize - 'a' as usize),
            _ => Lit(x.parse().unwrap()),
        }
    }

    let mut instrs = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["cpy", a, b] => Cpy(value(a), value(b)),
                ["inc", a] => Inc(a.chars().next().unwrap() as usize - 'a' as usize),
                ["dec", a] => Dec(a.chars().next().unwrap() as usize - 'a' as usize),
                ["tgl", a] => Tgl(a.chars().next().unwrap() as usize - 'a' as usize),
                ["out", a] => Out(value(a)),
                ["jnz", a, b] => Jnz(value(a), value(b)),
                _ => panic!("Invalid instruction {}", line),
            },
        )
        .collect::<Vec<_>>();
    optimize(&mut instrs);
    Sim {
        regs: [0; 4],
        line: 0,
        instrs,
    }
}

impl Sim {
    fn val(&self, v: &Value) -> i64 {
        match v {
            Reg(i) => self.regs[*i],
            Lit(n) => *n,
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        while self.line >= 0 && self.line < self.instrs.len() as i64 {
            match &self.instrs[self.line as usize] {
                Cpy(x, y) => {
                    if let Reg(i) = y {
                        self.regs[*i] = self.val(x);
                    }
                }
                Inc(r) => self.regs[*r] += 1,
                Dec(r) => self.regs[*r] -= 1,
                Tgl(r) => {
                    let i = self.line + self.regs[*r];
                    if i >= 0 && i < self.instrs.len() as i64 {
                        self.instrs[i as usize] = match &self.instrs[i as usize] {
                            Cpy(x, y) => Jnz(*x, *y),
                            Inc(r) => Dec(*r),
                            Dec(r) => Inc(*r),
                            Tgl(r) => Inc(*r),
                            Jnz(x, y) => Cpy(*x, *y),
                            _ => panic!("Invalid toggle"),
                        };
                    }
                }
                Out(v) => {
                    self.line += 1;
                    return Some(self.val(v));
                }
                Jnz(x, y) => {
                    if self.val(x) != 0 {
                        self.line += self.val(y) - 1;
                    }
                }
                Add(x, y) => {
                    self.regs[*x] += self.regs[*y];
                    self.regs[*y] = 0;
                }
                Mul(w, x, y, z) => {
                    self.regs[*y] += self.val(w) * self.regs[*x];
                    self.regs[*x] = 0;
                    self.regs[*z] = 0;
                }
                Nop => (),
            }
            self.line += 1;
        }
        None
    }
}

impl Iterator for Sim {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.run()
    }
}
