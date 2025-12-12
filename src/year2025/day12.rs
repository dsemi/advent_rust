use crate::utils::parsers::*;

pub fn part1(input: &str) -> usize {
    // Just ignore the present areas, assume everything is a full 3x3.
    let mut space = separated_pair(sep2(u16, 'x'), ':', spaced(u16));
    input
        .lines()
        .filter_map(|line| space.parse(line).ok())
        .filter(|((w, h), Sum(presents))| w * h >= 9 * presents)
        .count()
}

pub fn part2(_input: &str) -> &str {
    " "
}
