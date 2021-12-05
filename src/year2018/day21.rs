use ahash::AHashSet;
use std::cmp::max;

use crate::year2018::day19::Instr;
use crate::year2018::day19::Op::*;
use crate::year2018::day19::Prog;

impl Iterator for Prog {
    type Item = i64;

    #[rustfmt::skip]
    fn next(&mut self) -> Option<i64> {
        while self.reg[self.ip] >= 0 && self.reg[self.ip] < self.instrs.len() as i64 {
            if self.reg[self.ip] + 9 <= self.instrs.len() as i64 {
                match self.instrs[self.reg[self.ip] as usize..self.reg[self.ip] as usize + 9] {
                    [Instr(Seti, 0, _, t),
                     Instr(Addi, t2, 1, u),
                     Instr(Muli, u2, n, u3),
                     Instr(Gtrr, u4, r, u5),
                     Instr(Addr, u6, ip, ip2),
                     Instr(Addi, ip3, 1, ip4),
                     Instr(Seti, ipv8, _, ip5),
                     Instr(Addi, t3, u7, t4),
                     Instr(Seti, ipv, _, ip6)]
                        if t == t2
                            && t == t3
                            && t == t4
                            && u == u2
                            && u == u3
                            && u == u4
                            && u == u5
                            && u == u6
                            && u == u7
                            && self.ip as i64 == ip
                            && ip == ip2
                            && ip == ip3
                            && ip == ip4
                            && ip == ip5
                            && ip == ip6
                            && ipv8 == self.reg[self.ip] + 8
                            && ipv == self.reg[self.ip] =>
                    {
                        self.reg[u as usize] = 1;
                        self.reg[t as usize] = max(0, self.reg[r as usize] / n);
                        self.reg[self.ip] += 9;
                        continue;
                    }
                    _ => (),
                }
            }
            let ans = self.eval(self.instrs[self.reg[self.ip] as usize], true);
            self.reg[self.ip] += 1;
            if ans.is_some() {
                return ans;
            }
        }
        None
    }
}

pub fn part1(input: &str) -> Option<i64> {
    Prog::parse_instrs(input).next()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut s = AHashSet::new();
    Prog::parse_instrs(input)
        .take_while(|&x| s.insert(x))
        .last()
}
