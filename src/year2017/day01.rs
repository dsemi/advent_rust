fn solve(s: &str, n: usize) -> u32 {
    s.bytes()
        .cycle()
        .skip(n)
        .zip(s.bytes())
        .filter(|(a, b)| a == b)
        .map(|x| (x.0 - b'0') as u32)
        .sum()
}

pub fn part1(input: &str) -> u32 {
    solve(input, 1)
}

pub fn part2(input: &str) -> u32 {
    solve(input, input.len() / 2)
}
