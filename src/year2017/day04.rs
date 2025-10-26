use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let ps: Vec<&str> = line.split_whitespace().collect();
            ps.len() == ps.iter().unique().count()
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let ps: Vec<String> =
                line.split_whitespace().map(|x| x.chars().sorted().collect()).collect();
            ps.len() == ps.iter().unique().count()
        })
        .count()
}
