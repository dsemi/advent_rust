fn solve(s: &str, n: usize) -> u64 {
    let mut fish: Vec<u64> = ('0'..='8').map(|i| s.matches(i).count() as u64).collect();
    (0..n).for_each(|i| fish[(i + 7) % 9] += fish[i % 9]);
    fish.into_iter().sum()
}

pub fn part1(input: &str) -> u64 {
    solve(input, 80)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 256)
}
