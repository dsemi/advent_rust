use crate::utils::parsers::*;
use rayon::prelude::*;

#[derive(Clone)]
struct Num {
    depth: usize,
    value: u64,
}

struct Snailfish {
    ns: Vec<Num>,
}

fn parse<'a>(d: usize) -> impl Parser<&'a str, Vec<Num>, ContextError> {
    move |i: &mut &'a str| {
        alt((
            u64.map(|n| vec![Num { depth: d, value: n }]),
            delimited('[', list(parse(d + 1)), ']').map(|ns| ns.concat()),
        ))
        .parse_next(i)
    }
}

impl Snailfish {
    fn explode(&mut self) -> bool {
        for i in 0..self.ns.len() {
            if self.ns[i].depth > 4 {
                if i > 0 {
                    self.ns[i - 1].value += self.ns[i].value
                }
                if i + 2 < self.ns.len() {
                    self.ns[i + 2].value += self.ns[i + 1].value
                }
                self.ns.remove(i + 1);
                self.ns[i].value = 0;
                self.ns[i].depth -= 1;
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.ns.len() {
            if self.ns[i].value > 9 {
                let n = &self.ns[i];
                self.ns.insert(
                    i + 1,
                    Num {
                        depth: n.depth + 1,
                        value: (n.value + 1) / 2,
                    },
                );
                self.ns[i].value /= 2;
                self.ns[i].depth += 1;
                return true;
            }
        }
        false
    }

    fn magnitude(&mut self) -> u64 {
        while self.ns.len() > 1 {
            for i in 0..self.ns.len() - 1 {
                if self.ns[i].depth == self.ns[i + 1].depth {
                    self.ns[i] = Num {
                        depth: self.ns[i].depth - 1,
                        value: 3 * self.ns[i].value + 2 * self.ns[i + 1].value,
                    };
                    self.ns.remove(i + 1);
                    break;
                }
            }
        }
        self.ns[0].value
    }
}

fn add(a: &Snailfish, b: &Snailfish) -> Snailfish {
    let mut x = Snailfish {
        ns: a.ns.iter().chain(b.ns.iter()).cloned().collect(),
    };
    x.ns.iter_mut().for_each(|v| v.depth += 1);
    while x.explode() || x.split() {}
    x
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Snailfish {
            ns: parse(0).read(line),
        })
        .reduce(|a, b| add(&a, &b))
        .unwrap()
        .magnitude()
}

pub fn part2(input: &str) -> Option<u64> {
    let ns = input
        .lines()
        .map(|line| Snailfish {
            ns: parse(0).read(line),
        })
        .collect::<Vec<_>>();
    ns.par_iter()
        .flat_map(|a| ns.par_iter().map(|b| add(a, b).magnitude()))
        .max()
}
