use crate::utils::Mod;
use scan_fmt::scan_fmt as scanf;

#[derive(Clone, Copy)]
struct LinearTrans<const M: i64> {
    a: Mod<M>,
    b: Mod<M>,
}

impl<const M: i64> LinearTrans<M> {
    fn mappend(self, other: Self) -> Self {
        Self {
            a: other.a * self.a,
            b: other.a * self.b + other.b,
        }
    }

    fn invert(self) -> Self {
        let a = self.a.mod_inv();
        let b = -a * self.b;
        Self { a, b }
    }

    fn pow(self, n: i64) -> Self {
        assert!(n != 0);
        if n < 0 {
            return self.invert().pow(-n);
        }
        if n == 1 {
            return self;
        }
        if n.rem_euclid(2) == 0 {
            return self.mappend(self).pow(n.div_euclid(2));
        }
        self.mappend(self.pow(n - 1))
    }

    fn shuffle(self, n: i64, i: i64) -> i64 {
        let t2 = self.pow(n);
        (t2.a * Mod(i) + t2.b).0
    }
}

fn parse_techs<const M: i64>(input: &str) -> LinearTrans<M> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                LinearTrans {
                    a: Mod(M - 1),
                    b: Mod(M - 1),
                }
            } else if let Ok(n) = scanf!(line, "cut {}", i64) {
                LinearTrans {
                    a: Mod(1),
                    b: Mod((-n).rem_euclid(M)),
                }
            } else {
                let n = scanf!(line, "deal with increment {}", i64).unwrap();
                LinearTrans {
                    a: Mod(n.rem_euclid(M)),
                    b: Mod(0),
                }
            }
        })
        .reduce(|a, b| a.mappend(b))
        .unwrap()
}

pub fn part1(input: &str) -> i64 {
    parse_techs::<10007>(input).shuffle(1, 2019)
}

pub fn part2(input: &str) -> i64 {
    parse_techs::<119315717514047>(input).shuffle(-101741582076661, 2020)
}
