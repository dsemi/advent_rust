fn parse(input: &str) -> (u64, u64) {
    let pts: Vec<u64> = input
        .lines()
        .map(|x| x.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    (pts[0], pts[1])
}

fn judge(next_a: fn(u64) -> u64, next_b: fn(u64) -> u64, n: u64, gens: (u64, u64)) -> usize {
    let (mut a, mut b) = gens;
    let mut result = 0;
    for _ in 0..n {
        result += (a as u16 == b as u16) as usize;
        a = next_a(a);
        b = next_b(b);
    }
    result
}

fn next_a(a: u64) -> u64 {
    a * 16807 % 2147483647
}

fn next_b(b: u64) -> u64 {
    b * 48271 % 2147483647
}

pub fn part1(input: &str) -> usize {
    judge(next_a, next_b, 40_000_000, parse(input))
}

fn next_a2(a: u64) -> u64 {
    let mut result = next_a(a);
    while result % 4 != 0 {
        result = next_a(result);
    }
    result
}

fn next_b2(b: u64) -> u64 {
    let mut result = next_b(b);
    while result % 8 != 0 {
        result = next_b(result);
    }
    result
}

pub fn part2(input: &str) -> usize {
    judge(next_a2, next_b2, 5_000_000, parse(input))
}
