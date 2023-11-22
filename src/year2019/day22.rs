use scan_fmt::scan_fmt as scanf;

#[derive(Clone, Copy)]
struct LinearTrans<const M: i64> {
    a: i64,
    b: i64,
}

impl<const M: i64> LinearTrans<M> {
    fn mappend(self, other: Self) -> Self {
        Self {
            a: times::<M>(other.a, self.a),
            b: (times::<M>(other.a, self.b) + other.b).rem_euclid(M),
        }
    }

    fn invert(self) -> Self {
        let a = mod_inv(self.a, M);
        let b = times::<M>(-a, self.b);
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
        (t2.a * i + t2.b).rem_euclid(M)
    }
}

fn times<const M: i64>(mut a: i64, mut b: i64) -> i64 {
    let mut result = 0;
    while b > 0 {
        if b.rem_euclid(2) == 1 {
            result = (result + a).rem_euclid(M);
        }
        a = (2 * a).rem_euclid(M);
        b = b.div_euclid(2);
    }
    result
}

fn mod_inv(a0: i64, b0: i64) -> i64 {
    let (mut a, mut b, mut x0) = (a0, b0, 0);
    let mut result = 1;
    if b == 1 {
        return 1;
    }
    while a > 1 {
        result -= a.div_euclid(b) * x0;
        a = a.rem_euclid(b);
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x0, &mut result);
    }
    if result < 0 {
        result += b0;
    }
    result
}

fn parse_techs<const M: i64>(input: &str) -> LinearTrans<M> {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                LinearTrans { a: M - 1, b: M - 1 }
            } else if let Ok(n) = scanf!(line, "cut {}", i64) {
                LinearTrans {
                    a: 1,
                    b: (-n).rem_euclid(M),
                }
            } else {
                let n = scanf!(line, "deal with increment {}", i64).unwrap();
                LinearTrans {
                    a: n.rem_euclid(M),
                    b: 0,
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
