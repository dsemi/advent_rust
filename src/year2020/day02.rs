use crate::utils::parsers::*;

fn parse_line<'a>(i: &mut &'a [u8]) -> PResult<(usize, usize, u8, &'a [u8])> {
    let (a, _, b, _, c, _, d) = (usize, '-', usize, ' ', any, ": ", alpha1).parse_next(i)?;
    Ok((a, b, c, d))
}

fn count_valid(f: fn(usize, usize, u8, &[u8]) -> bool, input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_line.read(line.as_bytes()))
        .filter(|&(a, b, c, d)| f(a, b, c, d))
        .count()
}

pub fn part1(input: &str) -> usize {
    count_valid(
        |lo, hi, c, s| (lo..=hi).contains(&s.iter().filter(|&x| *x == c).count()),
        input,
    )
}

pub fn part2(input: &str) -> usize {
    count_valid(|lo, hi, c, s| (s[lo - 1] == c) != (s[hi - 1] == c), input)
}
