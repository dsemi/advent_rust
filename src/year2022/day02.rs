fn solve(input: &str, conv: fn(i32, i32) -> i32) -> i32 {
    input
        .lines()
        .map(|line| {
            let pts = line.as_bytes();
            let a = (pts[0] - b'A') as i32;
            let b = conv((pts[2] - b'X') as i32, a);
            let wld = if b == (a + 1) % 3 {
                6
            } else if b == a {
                3
            } else {
                0
            };
            wld + b + 1
        })
        .sum()
}

pub fn part1(input: &str) -> i32 {
    solve(input, |c, _| c)
}

pub fn part2(input: &str) -> i32 {
    solve(input, |c, a| (a + c - 1).rem_euclid(3))
}
