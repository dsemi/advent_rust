use crate::utils::parsers::*;

fn rotations(input: &str) -> impl Iterator<Item = i32> {
    let mut rot = (alt(('L'.value(-1), 'R'.value(1))), i32).map(|(d, n)| d * n);
    input.lines().map(move |line| rot.read(line))
}

pub fn part1(input: &str) -> i32 {
    rotations(input)
        .scan(50, |pos, n| {
            *pos += n;
            Some(i32::from(*pos % 100 == 0))
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    rotations(input)
        .scan(50, |pos, n| {
            let n_pos = *pos + n;
            let res = if n_pos > 0 { n_pos / 100 } else { i32::from(*pos != 0) - n_pos / 100 };
            *pos = n_pos.rem_euclid(100);
            Some(res)
        })
        .sum()
}
