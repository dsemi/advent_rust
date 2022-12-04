use scan_fmt::scan_fmt as scanf;

fn solve(input: &str, f: fn(i32, i32, i32, i32) -> bool) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a0, a1, b0, b1) = scanf!(line, "{}-{},{}-{}", i32, i32, i32, i32).unwrap();
            f(a0, a1, b0, b1)
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    solve(input, |a0, a1, b0, b1| {
        a0 <= b0 && a1 >= b1 || b0 <= a0 && b1 >= a1
    })
}

pub fn part2(input: &str) -> usize {
    solve(input, |a0, a1, b0, b1| a0 <= b1 && b0 <= a1)
}
