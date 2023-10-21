use ahash::AHashSet;
use Instr::*;
use Value::*;

#[derive(Clone, Copy, Debug)]
enum Value {
    Reg(usize),
    Lit(i64),
}

#[derive(Clone, Debug)]
enum Instr {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

#[derive(Clone)]
struct Prog {
    regs: [i64; 4],
    pc: usize,
}

fn parse(input: &str) -> Vec<Instr> {
    fn value(x: &str) -> Value {
        match x.as_bytes()[0] {
            c @ b'w'..=b'z' => Reg((c - b'w') as usize),
            _ => Lit(x.parse().unwrap()),
        }
    }
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                ["inp", a] => Inp((a.as_bytes()[0] - b'w') as usize),
                ["add", a, b] => Add((a.as_bytes()[0] - b'w') as usize, value(b)),
                ["mul", a, b] => Mul((a.as_bytes()[0] - b'w') as usize, value(b)),
                ["div", a, b] => Div((a.as_bytes()[0] - b'w') as usize, value(b)),
                ["mod", a, b] => Mod((a.as_bytes()[0] - b'w') as usize, value(b)),
                ["eql", a, b] => Eql((a.as_bytes()[0] - b'w') as usize, value(b)),
                _ => panic!("Invalid input: {}", line),
            },
        )
        .collect()
}

impl Prog {
    fn new() -> Self {
        Self {
            regs: [0; 4],
            pc: 0,
        }
    }

    fn val(&self, v: &Value) -> i64 {
        match v {
            Reg(r) => self.regs[*r],
            Lit(n) => *n,
        }
    }

    fn run_next(&mut self, instrs: &[Instr], inp: i64) -> bool {
        let mut a = 0;
        while self.pc < instrs.len() {
            match instrs[self.pc] {
                Inp(r) => self.regs[r] = inp,
                Add(r, v) => self.regs[r] += self.val(&v),
                Mul(r, v) => self.regs[r] *= self.val(&v),
                Div(3, Lit(v)) => {
                    a = v;
                    self.regs[3] /= v;
                }
                Div(r, v) => self.regs[r] /= self.val(&v),
                Mod(r, v) => self.regs[r] %= self.val(&v),
                Eql(r, v) => self.regs[r] = (self.regs[r] == self.val(&v)) as i64,
            }
            self.pc += 1;
            if matches!(instrs.get(self.pc), Some(Inp(_))) {
                break;
            }
        }
        assert!(a != 0);
        a != 26 || self.regs[1] == 0
    }

    fn dfs(
        &self,
        vis: &mut AHashSet<(i64, usize)>,
        instrs: &[Instr],
        n: i64,
        d: usize,
        p2: bool,
    ) -> Option<i64> {
        if d == 0 {
            return (self.regs[3] == 0).then_some(n);
        }
        if !vis.insert((self.regs[3], d)) {
            return None;
        }
        for i in (1..10).map(|n| if p2 { n } else { 10 - n }) {
            let mut p = self.clone();
            if !p.run_next(instrs, i) {
                continue;
            }
            if let Some(v) = p.dfs(vis, instrs, n * 10 + i, d - 1, p2) {
                return Some(v);
            }
        }
        None
    }
}

pub fn part1(input: &str) -> Option<i64> {
    Prog::new().dfs(&mut AHashSet::new(), &parse(input), 0, 14, false)
}

pub fn part2(input: &str) -> Option<i64> {
    Prog::new().dfs(&mut AHashSet::new(), &parse(input), 0, 14, true)
}
