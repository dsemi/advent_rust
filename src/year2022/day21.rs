use ahash::AHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64};
use nom::combinator::{map, value};
use nom::sequence::separated_pair;
use nom::IResult;
use num_complex::Complex;
use num_rational::Ratio;
use std::ops::{Add, Div, Mul, Sub};

type N = Complex<Ratio<i64>>;

enum Monkey<'a> {
    Num(N),
    Math(fn(N, N) -> N, &'a str, &'a str),
}

fn eval(m: &AHashMap<&str, Monkey>, k: &str) -> N {
    match &m[k] {
        Monkey::Num(n) => *n,
        Monkey::Math(op, l, r) => op(eval(m, l), eval(m, r)),
    }
}

fn num(i: &str) -> IResult<&str, Monkey> {
    map(i64, |n| Monkey::Num(Ratio::from_integer(n).into()))(i)
}

fn math(i: &str) -> IResult<&str, Monkey> {
    let (i, a) = alpha1(i)?;
    let (i, f) = alt((
        value(Add::add as fn(N, N) -> N, tag(" + ")),
        value(Sub::sub as fn(N, N) -> N, tag(" - ")),
        value(Mul::mul as fn(N, N) -> N, tag(" * ")),
        value(Div::div as fn(N, N) -> N, tag(" / ")),
    ))(i)?;
    let (i, b) = alpha1(i)?;
    Ok((i, Monkey::Math(f, a, b)))
}

fn parse(i: &str) -> IResult<&str, (&str, Monkey)> {
    separated_pair(alpha1, tag(": "), alt((num, math)))(i)
}

fn monkeys(input: &str) -> AHashMap<&str, Monkey> {
    input.lines().map(|line| parse(line).unwrap().1).collect()
}

pub fn part1(input: &str) -> i64 {
    *eval(&monkeys(input), "root").re.numer()
}

pub fn part2(input: &str) -> i64 {
    let mut m = monkeys(input);
    m.insert("humn", Monkey::Num(Complex::i()));
    if let Some(Monkey::Math(f, _, _)) = m.get_mut("root") {
        *f = Sub::sub;
    }
    let n = eval(&m, "root");
    *(-n.re / n.im).numer()
}
