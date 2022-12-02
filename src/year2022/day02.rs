fn solve(input: &str, conv: fn(u8, i32) -> i32) -> i32 {
    input
        .lines()
        .map(|line| {
            let pts = line.as_bytes();
            let a = (pts[0] - b'A') as i32;
            let b = conv(pts[2], a);
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
    solve(input, |c, _| match c {
        b'X' => 0,
        b'Y' => 1,
        b'Z' => 2,
        _ => unreachable!(),
    })
}

pub fn part2(input: &str) -> i32 {
    solve(input, |c, a| match c {
        b'X' => (a + 2) % 3,
        b'Y' => a,
        b'Z' => (a + 1) % 3,
        _ => unreachable!(),
    })
}
