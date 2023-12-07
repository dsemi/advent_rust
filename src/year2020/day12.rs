use crate::utils::parsers::*;
use crate::utils::C;

fn travel(start: C<i64>, move_way: bool, s: &str) -> i64 {
    let mut st = [C(0, 0), start];
    let idx = usize::from(move_way);
    for line in s.lines() {
        let cmd = line.chars().next().unwrap();
        let n = line[1..].i64();
        match cmd {
            'N' => st[idx] += C(0, n),
            'S' => st[idx] -= C(0, n),
            'E' => st[idx] += C(n, 0),
            'W' => st[idx] -= C(n, 0),
            'L' | 'R' => st[1] *= (if cmd == 'R' { C(0, -1) } else { C(0, 1) }).pow(n / 90),
            'F' => st[0] += st[1] * n,
            _ => panic!("Invalid instruction: {}", cmd),
        }
    }
    st[0].abs().sum()
}

pub fn part1(input: &str) -> i64 {
    travel(C(1, 0), false, input)
}

pub fn part2(input: &str) -> i64 {
    travel(C(10, 1), true, input)
}
