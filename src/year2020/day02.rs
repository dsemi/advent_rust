use crate::utils::parsers::*;

fn parse_line<'a>(i: &mut &'a str) -> PResult<(usize, usize, char, &'a str)> {
    let (a, b) = terminated(sep_tuple2(usize, '-'), space1).parse_next(i)?;
    let (c, d) = separated_pair(any, ": ", alpha1).parse_next(i)?;
    Ok((a, b, c, d))
}

fn count_valid(f: fn(usize, usize, char, &str) -> bool, input: &str) -> usize {
    lines_iter(input, parse_line)
        .filter(|&(a, b, c, d)| f(a, b, c, d))
        .count()
}

pub fn part1(input: &str) -> usize {
    count_valid(
        |lo, hi, c, s| {
            let count = s.matches(c).count();
            lo <= count && count <= hi
        },
        input,
    )
}

pub fn part2(input: &str) -> usize {
    count_valid(
        |lo, hi, c, s| {
            let chrs: Vec<char> = s.chars().collect();
            (chrs[lo - 1] == c) != (chrs[hi - 1] == c)
        },
        input,
    )
}
