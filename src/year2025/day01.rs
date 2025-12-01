use crate::utils::parsers::*;

fn rotations(input: &str) -> impl Iterator<Item = i32> {
    let mut rot = (alt(('L'.value(-1), 'R'.value(1))), i32).map(|(d, n)| d * n);
    input.lines().map(move |line| rot.read(line))
}

pub fn part1(input: &str) -> i32 {
    let mut pos = 50;
    let mut cnt = 0;
    for n in rotations(input) {
        pos = (pos + n).rem_euclid(100);
        cnt += i32::from(pos == 0);
    }
    cnt
}

pub fn part2(input: &str) -> i32 {
    let mut pos = 50;
    let mut cnt = 0;
    for n in rotations(input) {
        let next_pos = pos + n;
        if next_pos >= 100 {
            cnt += next_pos / 100;
        } else if next_pos <= 0 {
            cnt += i32::from(pos != 0) - next_pos / 100;
        }
        pos = next_pos.rem_euclid(100);
    }
    cnt
}
