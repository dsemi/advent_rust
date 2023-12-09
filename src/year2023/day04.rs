use crate::utils::parsers::*;

fn wins(i: &mut &str) -> PResult<usize> {
    ("Card", space1, u32, ':', space1).parse_next(i)?;
    let (win, own): (Vec<_>, Vec<_>) =
        sep2(separated(1.., u32, space1), (" |", space1)).parse_next(i)?;
    Ok(own.into_iter().filter(|o| win.contains(o)).count())
}

pub fn part1(input: &str) -> u32 {
    lines(wins)
        .read(input)
        .into_iter()
        .filter(|&n| n > 0)
        .map(|n| 1 << (n - 1))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let ns = lines(wins).read(input);
    let mut cards = vec![1; ns.len()];
    ns.into_iter()
        .enumerate()
        .for_each(|(i, n)| (1..=n).for_each(|j| cards[i + j] += cards[i]));
    cards.into_iter().sum()
}
