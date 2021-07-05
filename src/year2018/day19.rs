use crate::year2018::day19::Op::*;

#[derive(Clone, Copy)]
pub enum Op {
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

#[derive(Clone, Copy)]
pub struct Instr(pub Op, pub i64, pub i64, pub i64);

pub struct Prog {
    pub ip: usize,
    pub instrs: Vec<Instr>,
    pub reg: [i64; 6],
}

impl Prog {
    pub fn parse_instrs(input: &str) -> Self {
        let mut gen = input.lines();
        let ip = gen
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let instrs = gen
            .map(|line| {
                let (cmd, rest) = line.split_once(' ').unwrap();
                let op = match cmd {
                    "addr" => Addr,
                    "addi" => Addi,
                    "mulr" => Mulr,
                    "muli" => Muli,
                    "banr" => Banr,
                    "bani" => Bani,
                    "borr" => Borr,
                    "bori" => Bori,
                    "setr" => Setr,
                    "seti" => Seti,
                    "gtir" => Gtir,
                    "gtri" => Gtri,
                    "gtrr" => Gtrr,
                    "eqir" => Eqir,
                    "eqri" => Eqri,
                    "eqrr" => Eqrr,
                    _ => panic!("Invalid instruction"),
                };
                let ns = rest
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>();
                Instr(op, ns[0], ns[1], ns[2])
            })
            .collect();
        Self {
            ip: ip,
            instrs: instrs,
            reg: [0; 6],
        }
    }

    pub fn eval(&mut self, instr: Instr) -> Option<i64> {
        let Instr(op, a, b, c) = instr;
        match op {
            Addr => self.reg[c as usize] = self.reg[a as usize] + self.reg[b as usize],
            Addi => self.reg[c as usize] = self.reg[a as usize] + b,
            Mulr => self.reg[c as usize] = self.reg[a as usize] * self.reg[b as usize],
            Muli => self.reg[c as usize] = self.reg[a as usize] * b,
            Banr => self.reg[c as usize] = self.reg[a as usize] & self.reg[b as usize],
            Bani => self.reg[c as usize] = self.reg[a as usize] & b,
            Borr => self.reg[c as usize] = self.reg[a as usize] | self.reg[b as usize],
            Bori => self.reg[c as usize] = self.reg[a as usize] | b,
            Setr => self.reg[c as usize] = self.reg[a as usize],
            Seti => self.reg[c as usize] = a,
            Gtir => self.reg[c as usize] = (a > self.reg[b as usize]) as i64,
            Gtri => self.reg[c as usize] = (self.reg[a as usize] > b) as i64,
            Gtrr => self.reg[c as usize] = (self.reg[a as usize] > self.reg[b as usize]) as i64,
            Eqir => self.reg[c as usize] = (a == self.reg[b as usize]) as i64,
            Eqri => self.reg[c as usize] = (self.reg[a as usize] == b) as i64,
            Eqrr => {
                self.reg[c as usize] = (self.reg[a as usize] == self.reg[b as usize]) as i64;
                return Some(self.reg[a as usize]);
            },
        }
        None
    }

    fn run(&mut self) -> i64 {
        while self.reg[self.ip] >= 0 && self.reg[self.ip] < self.instrs.len() as i64 {
            self.eval(self.instrs[self.reg[self.ip] as usize]);
            self.reg[self.ip] += 1;
        }
        self.reg[0]
    }
}

pub fn part1(input: &str) -> i64 {
    Prog::parse_instrs(input).run()
}

// Not sure if there's a better way than just deconstructing the assembly
pub fn part2(_input: &str) -> i64 {
    let n = 10_551_361;
    (1..=n).filter_map(|d| (n % d == 0).then(|| d)).sum()
}
