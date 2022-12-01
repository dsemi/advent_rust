fn elves(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
}

pub fn part1(input: &str) -> Option<i32> {
    elves(input).max()
}

pub fn part2(input: &str) -> i32 {
    let mut elves = elves(input).collect::<Vec<_>>();
    elves.sort_unstable();
    elves[elves.len() - 3..].iter().sum()
}
