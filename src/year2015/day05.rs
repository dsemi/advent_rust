pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
                && line.as_bytes().array_windows().any(|[a, b]| a == b)
                && !["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x))
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|&line| {
            let bytes = line.as_bytes();
            (0..bytes.len() - 3).any(|i| {
                (i + 2..bytes.len() - 1)
                    .any(|j| bytes[i] == bytes[j] && bytes[i + 1] == bytes[j + 1])
            }) && bytes.array_windows().any(|[a, _, b]| a == b)
        })
        .count()
}
