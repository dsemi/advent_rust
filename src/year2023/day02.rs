use crate::utils::parsers::*;
use crate::utils::*;

fn color(i: &str) -> IResult<&str, C3<usize>> {
    alt((
        map(terminated(usize, tag(" red")), |r| C3(r, 0, 0)),
        map(terminated(usize, tag(" green")), |g| C3(0, g, 0)),
        map(terminated(usize, tag(" blue")), |b| C3(0, 0, b)),
    ))(i)
}

fn game(i: &str) -> IResult<&str, (usize, Vec<C3<usize>>)> {
    pair(
        delimited(tag("Game "), usize, tag(": ")),
        separated_list0(tag("; "), map(list(color), |v| v.into_iter().sum())),
    )(i)
}

fn parse(input: &str) -> impl Iterator<Item = (usize, Vec<C3<usize>>)> + '_ {
    input.lines().map(|line| game(line).unwrap().1)
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|(_, game)| {
            game.iter()
                .all(|&C3(r, g, b)| r <= 12 && g <= 13 && b <= 14)
        })
        .map(|(id, _)| id)
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|(_, game)| game.into_iter().reduce(C3::swol).unwrap().product())
        .sum()
}
