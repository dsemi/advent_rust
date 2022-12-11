use num::integer::lcm;

struct Op<'a>(&'a dyn Fn(usize, usize) -> usize, usize);

impl<'a> Op<'a> {
    fn apply(&self, b: usize) -> usize {
        self.0(self.1, b)
    }
}

struct Monkey<'a> {
    items: Vec<usize>,
    op: Op<'a>,
    divisor: usize,
    true_idx: usize,
    false_idx: usize,
}

fn parse_monkey(input: &str) -> Monkey<'_> {
    let lines = input.lines().collect::<Vec<_>>();
    let items = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect();
    let (oper, arg) = lines[2]
        .split_once("= old ")
        .unwrap()
        .1
        .split_once(' ')
        .unwrap();
    let op = if arg == "old" {
        Op(&|_, b| b * b, 0)
    } else if oper == "+" {
        let n = arg.parse().unwrap();
        Op(&|a, b| a + b, n)
    } else if oper == "*" {
        let n = arg.parse().unwrap();
        Op(&|a, b| a * b, n)
    } else {
        unreachable!();
    };
    let divisor = lines[3].rsplit_once(' ').unwrap().1.parse().unwrap();
    let true_idx = lines[4].rsplit_once(' ').unwrap().1.parse().unwrap();
    let false_idx = lines[5].rsplit_once(' ').unwrap().1.parse().unwrap();
    Monkey {
        items,
        op,
        divisor,
        true_idx,
        false_idx,
    }
}

fn solve(input: &str, p2: bool) -> usize {
    let mut mks = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    let m = mks.iter().map(|x| x.divisor).reduce(lcm).unwrap();
    let mut inspections = vec![0; mks.len()];
    let iters = if p2 { 10000 } else { 20 };
    for _ in 0..iters {
        for i in 0..mks.len() {
            inspections[i] += mks[i].items.len();
            for j in 0..mks[i].items.len() {
                let arg = mks[i].items[j];
                let mut worry_level = mks[i].op.apply(arg);
                if p2 {
                    worry_level %= m;
                } else {
                    worry_level /= 3;
                }
                let idx = if worry_level % mks[i].divisor == 0 {
                    mks[i].true_idx
                } else {
                    mks[i].false_idx
                };
                mks[idx].items.push(worry_level);
            }
            mks[i].items.clear();
        }
    }
    inspections.sort_unstable();
    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
