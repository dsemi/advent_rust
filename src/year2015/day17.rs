use crate::utils::Combinations;
use streaming_iterator::StreamingIterator;

fn all_combos(xs: Vec<i32>) -> impl Iterator<Item = usize> {
    (1..=xs.len()).map(move |n| {
        Combinations::new(&xs, n)
            .filter(|combo| combo.iter().copied().sum::<i32>() == 150)
            .count()
    })
}

pub fn part1(input: Vec<i32>) -> usize {
    all_combos(input).sum()
}

pub fn part2(input: Vec<i32>) -> Option<usize> {
    all_combos(input).find(|v| v > &0)
}
