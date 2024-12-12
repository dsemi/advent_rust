use crate::utils::parsers::*;
use advent::Parser;
use hashbrown::HashSet;
use Instr::*;
use Val::*;

#[derive(Clone, Copy, Debug, Parser)]
#[parser(dont_parse_name)]
enum Val {
    Reg(#[parser(impl = reg)] usize),
    Lit(i64),
}

fn reg(i: &mut &str) -> PResult<usize> {
    one_of(&['w', 'x', 'y', 'z'])
        .map(|c| c as usize - 'w' as usize)
        .parse_next(i)
}

#[derive(Clone, Debug, Parser)]
enum Instr {
    Inp(#[parser(impl = reg)] usize),
    Add(#[parser(impl = reg)] usize, Val),
    Mul(#[parser(impl = reg)] usize, Val),
    Div(#[parser(impl = reg)] usize, Val),
    Mod(#[parser(impl = reg)] usize, Val),
    Eql(#[parser(impl = reg)] usize, Val),
}

#[derive(Clone)]
struct Prog {
    regs: [i64; 4],
    pc: usize,
}

fn parse(input: &str) -> Vec<Instr> {
    lines(instr).read(input)
}

impl Prog {
    fn new() -> Self {
        Self {
            regs: [0; 4],
            pc: 0,
        }
    }

    fn val(&self, v: &Val) -> i64 {
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
        vis: &mut HashSet<(i64, usize)>,
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
    Prog::new().dfs(&mut HashSet::new(), &parse(input), 0, 14, false)
}

pub fn part2(input: &str) -> Option<i64> {
    Prog::new().dfs(&mut HashSet::new(), &parse(input), 0, 14, true)
}
