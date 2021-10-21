use itertools::iterate;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() / 3 - 2)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .flat_map(|line| {
            iterate(line.parse::<i64>().unwrap() / 3 - 2, |fuel| fuel / 3 - 2)
                .take_while(|fuel| fuel > &0)
        })
        .sum()
}
