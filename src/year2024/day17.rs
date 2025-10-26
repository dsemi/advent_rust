use crate::utils::parsers::*;

#[derive(Debug)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    instrs: Vec<i64>,
}

fn computer(i: &mut &str) -> Result<Computer> {
    let a = preceded("Register A: ", i64).parse_next(i)?;
    let b = preceded("\nRegister B: ", i64).parse_next(i)?;
    let c = preceded("\nRegister C: ", i64).parse_next(i)?;
    let instrs = preceded("\n\nProgram: ", list(i64)).parse_next(i)?;
    Ok(Computer { a, b, c, ip: 0, instrs })
}

impl Computer {
    fn reset(&mut self, a: i64) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
    }

    fn combo(&self, n: i64) -> i64 {
        match n {
            0..=3 => n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

impl Iterator for Computer {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ip < self.instrs.len() {
            let op = self.instrs[self.ip];
            let operand = self.instrs[self.ip + 1];
            match op {
                0 => self.a /= 1 << self.combo(operand),
                1 => self.b ^= operand,
                2 => self.b = self.combo(operand) % 8,
                3 if self.a != 0 => {
                    self.ip = operand as usize;
                    continue;
                }
                4 => self.b ^= self.c,
                5 => {
                    self.ip += 2;
                    return Some(self.combo(operand) % 8);
                }
                6 => self.b = self.a / (1 << self.combo(operand)),
                7 => self.c = self.a / (1 << self.combo(operand)),
                _ => (),
            }
            self.ip += 2;
        }
        None
    }
}

pub fn part1(input: &str) -> String {
    computer.read(input).map(|n| n.to_string()).collect::<Vec<_>>().join(",")
}

fn dfs(c: &mut Computer, idx: usize, a: i64) -> Option<i64> {
    (0..8).find_map(|i| {
        let a = (a << 3) | i;
        c.reset(a);
        (c.next().unwrap() == c.instrs[idx])
            .then(|| (idx == 0).then_some(a).or_else(|| dfs(c, idx - 1, a)))
            .flatten()
    })
}

pub fn part2(input: &str) -> Option<i64> {
    let mut comp = computer.read(input);
    let len = comp.instrs.len();
    dfs(&mut comp, len - 1, 0)
}
