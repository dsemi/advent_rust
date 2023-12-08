use crate::utils::parsers2::*;
use ahash::AHashMap;
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

fn num<'a>(i: &mut &'a str) -> PResult<Monkey<'a>> {
    i64.map(|n| Monkey::Num(Ratio::from_integer(n).into()))
        .parse_next(i)
}

fn math<'a>(i: &mut &'a str) -> PResult<Monkey<'a>> {
    let a = alpha1.parse_next(i)?;
    let f = alt((
        " + ".value(Add::add as fn(N, N) -> N),
        " - ".value(Sub::sub as fn(N, N) -> N),
        " * ".value(Mul::mul as fn(N, N) -> N),
        " / ".value(Div::div as fn(N, N) -> N),
    ))
    .parse_next(i)?;
    let b = alpha1.parse_next(i)?;
    Ok(Monkey::Math(f, a, b))
}

fn parse<'a>(i: &mut &'a str) -> PResult<(&'a str, Monkey<'a>)> {
    separated_pair(alpha1, ": ", alt((num, math))).parse_next(i)
}

fn monkeys(input: &str) -> AHashMap<&str, Monkey> {
    lines_iter(input, parse).collect()
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
