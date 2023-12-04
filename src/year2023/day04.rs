use crate::utils::parsers::*;

fn wins(i: &str) -> IResult<&str, usize> {
    let i = tuple((tag("Card"), space1, u32, tag(":"), space1))(i)?.0;
    let ns = |i| separated_list1(space1, u32)(i);
    let (i, (win, own)) = separated_pair(ns, pair(tag(" |"), space1), ns)(i)?;
    Ok((i, own.into_iter().filter(|o| win.contains(o)).count()))
}

pub fn part1(input: &str) -> u32 {
    lines(input, wins)
        .into_iter()
        .filter(|&n| n > 0)
        .map(|n| 1 << (n - 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let ns = lines(input, wins);
    let mut cards = vec![1; ns.len()];
    ns.into_iter()
        .enumerate()
        .for_each(|(i, n)| (1..=n).for_each(|j| cards[i + j] += cards[i]));
    cards.into_iter().sum()
}
