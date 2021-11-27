use scan_fmt::scan_fmt as scanf;

fn process(input: &str, f: fn((i32, i32, i32)) -> i32) -> i32 {
    input
        .lines()
        .map(|line| f(scanf!(line, "{}x{}x{}", i32, i32, i32).unwrap()))
        .sum()
}

pub fn part1(input: &str) -> i32 {
    process(input, |(l, w, h)| {
        2 * (l * w + l * h + w * h) + [l * w, l * h, w * h].iter().min().unwrap()
    })
}

pub fn part2(input: &str) -> i32 {
    process(input, |(l, w, h)| {
        l * w * h + 2 * [l + w, l + h, w + h].iter().min().unwrap()
    })
}
