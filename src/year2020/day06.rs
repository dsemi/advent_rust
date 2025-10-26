fn solve(f: fn(u32, u32) -> u32, s: &str) -> u32 {
    s.split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|x| {
                    x.chars().map(|c| 1 << (c as u32 - 'a' as u32)).reduce(|a, b| a | b).unwrap()
                })
                .reduce(f)
                .unwrap()
                .count_ones()
        })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    solve(|a, b| a | b, input)
}

pub fn part2(input: &str) -> u32 {
    solve(|a, b| a & b, input)
}
