fn solve(input: &str, n: usize) -> u64 {
    let mut fish = [0; 9];
    input
        .split(',')
        .for_each(|n| fish[n.parse::<usize>().unwrap()] += 1);
    (0..n).for_each(|i| fish[(i + 7) % 9] += fish[i % 9]);
    fish.into_iter().sum()
}

pub fn part1(input: &str) -> u64 {
    solve(input, 80)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 256)
}
