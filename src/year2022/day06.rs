use ahash::AHashSet;

fn solve(input: &str, nchars: usize) -> Option<usize> {
    (0..input.len()).find_map(|i| {
        (input[i..i + nchars].chars().collect::<AHashSet<_>>().len() == nchars).then(|| i + nchars)
    })
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, 14)
}
