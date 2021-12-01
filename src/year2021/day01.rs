fn count_increasing<I: Iterator<Item = i32>>(ns: I) -> usize {
    ns.collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

pub fn part1(input: &str) -> usize {
    count_increasing(input.lines().map(|x| x.parse().unwrap()))
}

pub fn part2(input: &str) -> usize {
    count_increasing(
        input
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>()
            .windows(3)
            .map(|w| w.iter().sum()),
    )
}
