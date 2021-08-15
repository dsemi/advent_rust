#[derive(Clone, Copy)]
struct LinearTrans {
    a: i64,
    b: i64,
    modulus: i64,
}

impl LinearTrans {
    fn mappend(self, other: Self) -> Self {
        assert!(self.modulus == other.modulus);
        Self {
            a: times(self.modulus, other.a, self.a),
            b: (times(self.modulus, other.a, self.b) + other.b).rem_euclid(self.modulus),
            modulus: self.modulus,
        }
    }

    fn invert(self) -> Self {
        let a = mod_inv(self.a, self.modulus);
        let b = times(self.modulus, -a, self.b);
        Self {
            a,
            b,
            modulus: self.modulus,
        }
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
        (t2.a * i + t2.b).rem_euclid(t2.modulus)
    }
}

fn times(m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut result = 0;
    while b > 0 {
        if b.rem_euclid(2) == 1 {
            result = (result + a).rem_euclid(m);
        }
        a = (2 * a).rem_euclid(m);
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

fn parse_techs(input: &str, modulus: i64) -> LinearTrans {
    input
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                LinearTrans {
                    a: modulus - 1,
                    b: modulus - 1,
                    modulus,
                }
            } else if let Ok(n) = scan_fmt!(line, "cut {}", i64) {
                LinearTrans {
                    a: 1,
                    b: (-n).rem_euclid(modulus),
                    modulus,
                }
            } else {
                let n = scan_fmt!(line, "deal with increment {}", i64).unwrap();
                LinearTrans {
                    a: n.rem_euclid(modulus),
                    b: 0,
                    modulus,
                }
            }
        })
        .reduce(|a, b| a.mappend(b))
        .unwrap()
}

pub fn part1(input: &str) -> i64 {
    let modulus = 10007;
    parse_techs(input, modulus).shuffle(1, 2019)
}

pub fn part2(input: &str) -> i64 {
    let modulus = 119315717514047;
    parse_techs(input, modulus).shuffle(-101741582076661, 2020)
}
