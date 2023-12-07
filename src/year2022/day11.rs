use crate::utils::parsers::*;
use num::integer::lcm;
use rayon::prelude::*;
use Op::*;

#[derive(Clone)]
enum Op {
    Double,
    Add(u64),
    Mul(u64),
}

impl Op {
    fn parse(i: &str) -> IResult<&str, Op> {
        alt((
            value(Double, tag("old * old")),
            map(preceded(tag("old + "), u64), Add),
            map(preceded(tag("old * "), u64), Mul),
        ))(i)
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

fn monkey(i: &str) -> IResult<&str, Monkey> {
    let i = delimited(tag("Monkey "), usize, tag(":"))(i)?.0;
    let (i, ns) = preceded(tag("\n  Starting items: "), list(u64))(i)?;
    let (i, op) = preceded(tag("\n  Operation: new = "), Op::parse)(i)?;
    let (i, test) = preceded(tag("\n  Test: divisible by "), u64)(i)?;
    let (i, t) = preceded(tag("\n    If true: throw to monkey "), usize)(i)?;
    let (i, f) = preceded(tag("\n    If false: throw to monkey "), usize)(i)?;
    Ok((i, Monkey { ns, op, test, t, f }))
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
        let i2 = if v % monkey.test == 0 {
            monkey.t
        } else {
            monkey.f
        };
        round += (i2 < i) as usize;
        inspections[i] += 1;
        i = i2;
    }
    inspections
}

fn solve(input: &str, p2: bool) -> usize {
    let mks = sep_list(tag("\n\n"), monkey).read(input);
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
        items
            .into_par_iter()
            .map(|item| play(&mks, 10000, item, |x| x % m))
            .reduce(
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
