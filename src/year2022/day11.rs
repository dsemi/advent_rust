use crate::utils::parsers::*;
use Op::*;
use num::integer::lcm;
use rayon::prelude::*;

#[derive(Clone)]
enum Op {
    Double,
    Add(u64),
    Mul(u64),
}

impl Op {
    fn parse(i: &mut &str) -> ModalResult<Op> {
        alt((
            "old * old".value(Double),
            preceded("old + ", u64).map(Add),
            preceded("old * ", u64).map(Mul),
        ))
        .parse_next(i)
    }

    fn ap(&self, x: u64) -> u64 {
        match self {
            Double => x * x,
            Add(y) => x + y,
            Mul(y) => x * y,
        }
    }
}

struct Monkey {
    ns: Vec<u64>,
    op: Op,
    test: u64,
    t: usize,
    f: usize,
}

fn monkey(i: &mut &str) -> ModalResult<Monkey> {
    delimited("Monkey ", usize, ':').parse_next(i)?;
    let ns = preceded("\n  Starting items: ", list(u64)).parse_next(i)?;
    let op = preceded("\n  Operation: new = ", Op::parse).parse_next(i)?;
    let test = preceded("\n  Test: divisible by ", u64).parse_next(i)?;
    let t = preceded("\n    If true: throw to monkey ", usize).parse_next(i)?;
    let f = preceded("\n    If false: throw to monkey ", usize).parse_next(i)?;
    Ok(Monkey { ns, op, test, t, f })
}

fn play(
    monkeys: &[Monkey],
    rounds: usize,
    item: (usize, u64),
    adjust: impl Fn(u64) -> u64,
) -> Vec<usize> {
    let mut inspections = vec![0; monkeys.len()];
    let mut round = 0;
    let (mut i, mut v) = item;
    while round < rounds {
        let monkey = &monkeys[i];
        let worry = monkey.op.ap(v);
        v = adjust(worry);
        let i2 = if v % monkey.test == 0 { monkey.t } else { monkey.f };
        round += (i2 < i) as usize;
        inspections[i] += 1;
        i = i2;
    }
    inspections
}

fn solve(input: &str, p2: bool) -> usize {
    let mks: Vec<_> = separated(1.., monkey, "\n\n").read(input);
    let items = mks
        .iter()
        .enumerate()
        .flat_map(|(i, m)| m.ns.iter().map(move |&n| (i, n)))
        .collect::<Vec<_>>();
    let mut inspections = if !p2 {
        items
            .into_iter()
            .map(|item| play(&mks, 20, item, |x| x / 3))
            .reduce(|a, b| a.into_iter().zip(b).map(|(a, b)| a + b).collect())
            .unwrap()
    } else {
        let m = mks.iter().map(|m| m.test).reduce(lcm).unwrap();
        items.into_par_iter().map(|item| play(&mks, 10000, item, |x| x % m)).reduce(
            || vec![0; mks.len()],
            |a, b| a.into_iter().zip(b).map(|(a, b)| a + b).collect(),
        )
    };
    inspections.sort_unstable();
    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
