use crate::utils::parsers::*;
use crate::utils::*;

fn color(i: &str) -> IResult<&str, C3<usize>> {
    alt((
        map(terminated(usize, tag(" red")), |r| C3(r, 0, 0)),
        map(terminated(usize, tag(" green")), |g| C3(0, g, 0)),
        map(terminated(usize, tag(" blue")), |b| C3(0, 0, b)),
    ))(i)
}

fn game(i: &str) -> C3<usize> {
    i.split("; ")
        .map(|roll| roll.split(", ").map(|c| color.read(c)).sum())
        .reduce(C3::swol)
        .unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = C3<usize>> + '_ {
    input
        .lines()
        .map(|line| game(line.split_once(": ").unwrap().1))
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .enumerate()
        .filter(|&(_, C3(r, g, b))| r <= 12 && g <= 13 && b <= 14)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input).map(|game| game.product()).sum()
}
