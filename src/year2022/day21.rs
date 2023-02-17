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

fn monkeys(input: &str) -> AHashMap<&str, Monkey> {
    input
        .lines()
        .map(|line| {
            let pts = line.split_whitespace().collect::<Vec<_>>();
            let monkey = match pts[1..] {
                [n] => Monkey::Num(n.parse().unwrap()),
                [a, op, b] => {
                    let f = match op.chars().next().unwrap() {
                        '+' => Add::add,
                        '-' => Sub::sub,
                        '*' => Mul::mul,
                        '/' => Div::div,
                        _ => panic!("Bad op: {}", op),
                    };
                    Monkey::Math(f, a, b)
                }
                _ => panic!("Bad parse: {}", line),
            };
            (&pts[0][..pts[0].len() - 1], monkey)
        })
        .collect()
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
