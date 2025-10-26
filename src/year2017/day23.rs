use crate::utils::parsers::*;
use Instr::*;
use Val::*;
use advent::Parser;

#[derive(Parser)]
#[parser(dont_parse_name)]
enum Val {
    Lit(i64),
    Reg(#[parser(impl = reg)] usize),
}

fn reg(i: &mut &str) -> ModalResult<usize> {
    any.map(|c| c as usize - 'a' as usize).parse_next(i)
}

#[derive(Parser)]
enum Instr {
    Set(#[parser(impl = reg)] usize, Val),
    Sub(#[parser(impl = reg)] usize, Val),
    Mul(#[parser(impl = reg)] usize, Val),
    Jnz(Val, Val),
}

struct Prog {
    reg: [i64; 8],
    line: i64,
    instrs: Vec<Instr>,
}

fn parse_instrs(input: &str) -> Prog {
    let instrs = lines(instr).read(input);
    Prog { reg: [0; 8], line: 0, instrs }
}

fn is_prime(n: i64) -> bool {
    for i in std::iter::once(2).chain((3..(n as f64).sqrt() as i64 + 1).step_by(2)) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

impl Prog {
    fn val(&self, arg: &Val) -> i64 {
        match arg {
            Lit(v) => *v,
            Reg(r) => self.reg[*r],
        }
    }

    #[rustfmt::skip]
    fn run(&mut self, debug: bool) -> usize {
        let mut result = 0;
        while self.line >= 0 && self.line < self.instrs.len() as i64 {
            if !debug && self.line + 14 <= self.instrs.len() as i64 {
                match self.instrs[self.line as usize..self.line as usize + 14] {
                    [Set(e, Lit(2)),
                     Set(g, Reg(d)),
                     Mul(g2, Reg(e2)),
                     Sub(g3, Reg(b)),
                     Jnz(Reg(g4), Lit(2)),
                     Set(f, Lit(0)),
                     Sub(e3, Lit(-1)),
                     Set(g5, Reg(e4)),
                     Sub(g6, Reg(b2)),
                     Jnz(Reg(g7), Lit(-8)),
                     Sub(d2, Lit(-1)),
                     Set(g8, Reg(d3)),
                     Sub(g9, Reg(b3)),
                     Jnz(Reg(g10), Lit(-13))]
                        if b == b2
                            && b == b3
                            && d == d2
                            && d == d3
                            && e == e2
                            && e == e3
                            && e == e4
                            && g == g2
                            && g == g3
                            && g == g4
                            && g == g5
                            && g == g6
                            && g == g7
                            && g == g8
                            && g == g9
                            && g == g10 =>
                    {
                        let (to_check, inner_counter, outer_counter, workspace, prime_check) =
                            (b, e, d, g, f);
                        let v = self.reg[to_check];
                        self.reg[inner_counter] = v;
                        self.reg[outer_counter] = v;
                        self.reg[workspace] = 0;
                        self.reg[prime_check] = is_prime(v) as i64;
                        self.line += 14;
                        continue;
                    }
                    _ => (),
                }
            }
            match &self.instrs[self.line as usize] {
                Set(r, v) => self.reg[*r] = self.val(v),
                Sub(r, v) => self.reg[*r] -= self.val(v),
                Mul(r, v) => {
                    if debug {
                        result += 1;
                    }
                    self.reg[*r] *= self.val(v);
                }
                Jnz(a, b) => {
                    if self.val(a) != 0 {
                        self.line += self.val(b) - 1;
                    }
                }
            }
            self.line += 1;
        }
        result
    }
}

pub fn part1(input: &str) -> usize {
    let mut prog = parse_instrs(input);
    prog.run(true)
}

pub fn part2(input: &str) -> i64 {
    let mut prog = parse_instrs(input);
    prog.reg[0] = 1;
    prog.run(false);
    prog.reg[7]
}
