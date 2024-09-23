use crate::utils::parsers::*;
use crate::utils::DefaultVec;
use advent::Parser;

#[derive(Clone, Copy, Parser)]
enum Loc {
    Output(usize),
    Bot(usize),
}

#[derive(Clone, Copy, Default)]
struct Bot(usize, [i64; 2], Option<(Loc, Loc)>);

impl Bot {
    fn recv(&mut self, val: i64) {
        self.1[self.0] = val;
        self.0 += 1;
    }
}

#[derive(Default)]
struct Factory {
    bots: DefaultVec<Bot>,
    outs: DefaultVec<i64>,
}

impl Factory {
    fn run(input: &str) -> Self {
        let mut f = Self::default();
        let mut bot_pat = ("bot ", usize, " gives low to ", loc, " and high to ", loc);
        for line in input.lines() {
            if let Ok((_, v, _, t)) = ("value ", i64, " goes to ", loc).parse(line) {
                f.send(v, t);
            } else if let Ok((_, n, _, lo, _, hi)) = bot_pat.parse(line) {
                f.bots.get_mut(n).2 = Some((lo, hi));
                f.run_bot(n);
            }
        }
        f
    }

    fn send(&mut self, val: i64, loc: Loc) {
        match loc {
            Loc::Output(i) => *self.outs.get_mut(i) = val,
            Loc::Bot(i) => {
                self.bots.get_mut(i).recv(val);
                self.run_bot(i);
            }
        }
    }

    fn run_bot(&mut self, idx: usize) {
        if let Bot(2, [a, b], Some((lo, hi))) = *self.bots.get(idx) {
            self.send(a.min(b), lo);
            self.send(a.max(b), hi);
        }
    }
}

pub fn part1(input: &str) -> Option<usize> {
    Factory::run(input)
        .bots
        .iter()
        .enumerate()
        .find(|&(_, &Bot(_, vals, _))| vals == [17, 61] || vals == [61, 17])
        .map(|x| x.0)
}

pub fn part2(input: &str) -> i64 {
    Factory::run(input).outs.iter().take(3).product()
}
