use crate::utils::parsers::*;
use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let (mut ls, mut rs): (Vec<_>, Vec<_>) =
        input.lines().map(|line| sep2(usize, space1).read(line)).unzip();
    ls.sort_unstable();
    rs.sort_unstable();
    ls.into_iter().zip(rs).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> usize {
    let (ls, rs): (Vec<_>, Vec<_>) =
        input.lines().map(|line| sep2(usize, space1).read(line)).unzip();
    let cnts = rs.into_iter().counts();
    ls.into_iter().map(|n| n * *cnts.get(&n).unwrap_or(&0)).sum()
}
