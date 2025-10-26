use crate::utils::*;

pub fn part1(input: &str) -> String {
    transpose(&input.lines().map(|x| x.chars().collect()).collect::<Vec<_>>())
        .into_iter()
        .map(|row| row.into_iter().most_common().unwrap())
        .collect()
}

pub fn part2(input: &str) -> String {
    transpose(&input.lines().map(|x| x.chars().collect()).collect::<Vec<_>>())
        .into_iter()
        .map(|row| row.into_iter().least_common().unwrap())
        .collect()
}
