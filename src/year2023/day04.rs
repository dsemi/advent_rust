use crate::utils::parsers::*;

fn winning_nums(i: &str) -> IResult<&str, usize> {
    let i = tuple((tag("Card"), space1, u32, tag(":"), space1))(i)?.0;
    let ns = |i| separated_list1(space1, u32)(i);
    let (i, (win, own)) = separated_pair(ns, pair(tag(" |"), space1), ns)(i)?;
    Ok((i, own.into_iter().filter(|o| win.contains(o)).count()))
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| winning_nums(line).unwrap().1)
        .collect()
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .filter(|&n| n > 0)
        .map(|n| 1 << (n - 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let ns = parse(input);
    let mut scratchcards = vec![1; ns.len()];
    for (i, n) in ns.into_iter().enumerate() {
        for j in i + 1..=i + n {
            scratchcards[j] += scratchcards[i];
        }
    }
    scratchcards.into_iter().sum()
}
