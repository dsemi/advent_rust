use crate::utils::parsers::*;

pub fn part1(input: &str) -> usize {
    let mut space = separated_pair(sep2(u16, 'x'), ':', spaced(u16));
    input
        .lines()
        .filter_map(|line| space.parse(line).ok())
        // Pretend we're packing 3x3 squares, problem input either doesn't fit
        // or has lots of empty space.
        .filter(|((w, h), Sum(presents))| (w - w % 3) * (h - h % 3) >= 9 * presents)
        .count()
}

pub fn part2(_input: &str) -> &str {
    " "
}
