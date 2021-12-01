use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}
