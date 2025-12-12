use crate::utils::parsers::*;

pub fn part1(input: &str) -> usize {
    let (presents, spaces) = input.rsplit_once("\n\n").unwrap();
    let areas: Vec<_> =
        presents.split("\n\n").map(|p| p.chars().filter(|&c| c == '#').count() as u16).collect();
    spaces
        .lines()
        .map(|space| separated_pair(sep2(u16, 'x'), ':', spaced(u16)).read(space))
        .filter(|((w, h), ns)| w * h >= ns.into_iter().zip(&areas).map(|(n, a)| n * a).sum())
        .count()
}

pub fn part2(_input: &str) -> &str {
    " "
}
