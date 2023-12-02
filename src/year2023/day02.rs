use crate::utils::parsers::*;
use crate::utils::*;

fn color(i: &str) -> IResult<&str, C3<usize>> {
    alt((
        map(terminated(usize, tag(" red")), |r| C3(r, 0, 0)),
        map(terminated(usize, tag(" green")), |g| C3(0, g, 0)),
        map(terminated(usize, tag(" blue")), |b| C3(0, 0, b)),
    ))(i)
}

fn game(i: &str) -> IResult<&str, C3<usize>> {
    map(
        separated_list0(tag("; "), map(list(color), |v| v.into_iter().sum())),
        |v| v.into_iter().reduce(C3::swol).unwrap(),
    )(i)
}

fn parse(input: &str) -> impl Iterator<Item = C3<usize>> + '_ {
    input
        .lines()
        .map(|line| game(line.split_once(": ").unwrap().1).unwrap().1)
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
