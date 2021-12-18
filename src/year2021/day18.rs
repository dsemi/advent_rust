use crate::utils::int;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use rayon::prelude::*;
use Snailfish::*;

#[derive(Clone)]
enum Snailfish {
    Reg(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            |i| int(i).map(|(i, n)| (i, Reg(n))),
            |i| {
                delimited(
                    tag("["),
                    separated_pair(Self::parse, tag(","), Self::parse),
                    tag("]"),
                )(i)
                .map(|(i, (a, b))| (i, Pair(Box::new(a), Box::new(b))))
            },
        ))(i)
    }

    fn explode(&mut self) -> bool {
        let mut next = None;
        self.exp(&mut None, &mut next, 0) || next.is_some()
    }

    fn exp<'a>(
        &'a mut self,
        prev: &mut Option<&'a mut u64>,
        next: &mut Option<u64>,
        depth: usize,
    ) -> bool {
        match self {
            Reg(n) if next.is_some() => {
                *n += next.unwrap();
                return true;
            }
            Pair(a, b) if next.is_none() && depth == 4 => {
                if let Some(p) = prev.take() {
                    if let Reg(v) = **a {
                        *p += v;
                    }
                }
                if let Reg(v) = **b {
                    *next = Some(v);
                }
                *self = Reg(0);
            }
            Reg(n) => *prev = Some(n),
            Pair(a, b) => {
                return a.exp(prev, next, depth + 1) || b.exp(prev, next, depth + 1);
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        match self {
            Reg(r) if *r > 9 => {
                *self = Pair(Box::new(Reg(*r / 2)), Box::new(Reg((*r + 1) / 2)));
                true
            }
            Pair(a, b) => a.split() || b.split(),
            _ => false,
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Reg(n) => *n,
            Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }
}

fn add(a: Snailfish, b: Snailfish) -> Snailfish {
    let mut x = Pair(Box::new(a), Box::new(b));
    while x.explode() || x.split() {}
    x
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Snailfish::parse(line).unwrap().1)
        .reduce(add)
        .unwrap()
        .magnitude()
}

pub fn part2(input: &str) -> Option<u64> {
    let ns = input
        .lines()
        .map(|line| Snailfish::parse(line).unwrap().1)
        .collect::<Vec<_>>();
    ns.par_iter()
        .flat_map(|a| ns.par_iter().map(|b| add(a.clone(), b.clone()).magnitude()))
        .max()
}
