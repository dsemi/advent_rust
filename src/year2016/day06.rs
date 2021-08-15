use counter::Counter;

use crate::utils::*;

pub fn part1(input: &str) -> String {
    transpose(
        &input
            .lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .map(|row| row.into_iter().collect::<Counter<_>>().most_common()[0].0)
    .collect()
}

pub fn part2(input: &str) -> String {
    transpose(
        &input
            .lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .map(|row| {
        let c = row.into_iter().collect::<Counter<_>>();
        c.most_common()[c.len() - 1].0
    })
    .collect()
}
