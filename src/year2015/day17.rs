use streaming_iterator::StreamingIterator;

use crate::utils::Combinations;

fn all_combos(input: &str) -> impl Iterator<Item = usize> {
    let xs: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    (1..=xs.len()).map(move |n| {
        Combinations::new(&xs, n)
            .filter(|combo| combo.iter().copied().sum::<i32>() == 150)
            .count()
    })
}

pub fn part1(input: &str) -> usize {
    all_combos(input).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    all_combos(input).find(|v| v > &0)
}
