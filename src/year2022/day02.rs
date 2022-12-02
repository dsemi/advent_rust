fn solve(input: &str, f: fn(i32, i32) -> i32) -> i32 {
    input
        .lines()
        .map(|line| {
            let pts = line.as_bytes();
            f((pts[0] - b'A') as i32, (pts[2] - b'X') as i32)
        })
        .sum()
}

pub fn part1(input: &str) -> i32 {
    solve(input, |a, b| 3 * (b - a + 1).rem_euclid(3) + b + 1)
}

pub fn part2(input: &str) -> i32 {
    solve(input, |a, b| (a + b - 1).rem_euclid(3) + 3 * b + 1)
}
