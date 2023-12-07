use crate::utils::parsers::*;

fn parse_line(i: &str) -> IResult<&str, (usize, usize, char, &str)> {
    let (i, (a, b)) = terminated(sep_tuple2(tag("-"), usize), space1)(i)?;
    let (i, (c, d)) = separated_pair(anychar, tag(": "), alpha1)(i)?;
    Ok((i, (a, b, c, d)))
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
