use ahash::AHashMap;
use Monkey::*;
use Op::*;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Num(i64),
    Math(String, Op, String),
}

fn eval(op: Op, left: i64, right: i64) -> i64 {
    match op {
        Add => left + right,
        Sub => left - right,
        Mul => left * right,
        Div => left / right,
    }
}

fn monkeys(input: &str) -> AHashMap<String, Monkey> {
    input
        .lines()
        .map(|line| {
            let pts = line.split_whitespace().collect::<Vec<_>>();
            let monkey = match pts[1..] {
                [n] => Num(n.parse().unwrap()),
                [a, op, b] => Math(
                    a.to_owned(),
                    match op.chars().next().unwrap() {
                        '+' => Add,
                        '-' => Sub,
                        '*' => Mul,
                        '/' => Div,
                        _ => panic!("Bad op: {}", op),
                    },
                    b.to_owned(),
                ),
                _ => panic!("Bad parse {}", line),
            };
            (pts[0][..pts[0].len() - 1].to_owned(), monkey)
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let ms = monkeys(input);
    fn val(m: &AHashMap<String, Monkey>, k: &str) -> i64 {
        match &m[k] {
            Num(n) => *n,
            Math(a, op, b) => eval(*op, val(m, a), val(m, b)),
        }
    }
    val(&ms, "root")
}

pub fn part2(input: &str) -> i64 {
    let ms = monkeys(input);
    fn val(m: &AHashMap<String, Monkey>, k: &str) -> Result<i64, Box<dyn Fn(i64) -> i64>> {
        if k == "humn" {
            return Err(Box::new(|x| x));
        }
        match &m[k] {
            Num(n) => Ok(*n),
            Math(a, op, b) => {
                let left = val(m, a);
                let right = val(m, b);
                match (left, right, op) {
                    (Err(f), Ok(n), Add) => Err(Box::new(move |x| f(x - n))),
                    (Err(f), Ok(n), Sub) => Err(Box::new(move |x| f(x + n))),
                    (Err(f), Ok(n), Mul) => Err(Box::new(move |x| f(x / n))),
                    (Err(f), Ok(n), Div) => Err(Box::new(move |x| f(x * n))),
                    (Ok(n), Err(f), Add) => Err(Box::new(move |x| f(x - n))),
                    (Ok(n), Err(f), Sub) => Err(Box::new(move |x| f(n - x))),
                    (Ok(n), Err(f), Mul) => Err(Box::new(move |x| f(x / n))),
                    (Ok(n), Err(f), Div) => Err(Box::new(move |x| f(n / x))),
                    (Ok(a), Ok(b), _) => Ok(eval(*op, a, b)),
                    _ => unreachable!(),
                }
            }
        }
    }
    if let Math(l, _, r) = &ms["root"] {
        return match (val(&ms, l), val(&ms, r)) {
            (Err(f), Ok(n)) => f(n),
            (Ok(n), Err(f)) => f(n),
            _ => unreachable!(),
        };
    }
    unreachable!()
}
