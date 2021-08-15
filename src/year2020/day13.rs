fn chinese_remainder(an: Vec<(i64, i64)>) -> i64 {
    let prod: i64 = an.iter().map(|(_, x)| x).product();
    let mut sum = 0;
    for (a_i, n_i) in an {
        let p = prod / n_i;
        sum += a_i * mul_inv(p, n_i) * p;
        sum = sum.rem_euclid(prod);
    }
    sum
}

fn mul_inv(a: i64, b: i64) -> i64 {
    let mut ab = (a, b);
    let b0 = b;
    let mut x0x1 = (0, 1);
    if b == 1 {
        return 1;
    }
    while ab.0 > 1 {
        let (a, b) = ab;
        let q = a / b;
        ab = (b, a % b);
        let (x0, x1) = x0x1;
        x0x1 = (x1 - q * x0, x0);
    }
    if x0x1.1 < 0 {
        x0x1.1 += b0;
    }
    x0x1.1
}

fn parse_buses(s: &str) -> (i64, Vec<(i64, i64)>) {
    let ls: Vec<&str> = s.lines().collect();
    let t = ls[0].parse().unwrap();
    (
        t,
        ls[1]
            .split(',')
            .enumerate()
            .filter(|&(_, x)| x != "x")
            .map(|(i, x)| (-(i as i64), x.parse().unwrap()))
            .collect(),
    )
}

pub fn part1(input: &str) -> i64 {
    let (t, buses) = parse_buses(input);
    let (a, b) = buses.iter().map(|(_, b)| (b, b - t % b)).min_by_key(|x| x.1).unwrap();
    a * b
}

pub fn part2(input: &str) -> i64 {
    chinese_remainder(parse_buses(input).1)
}
