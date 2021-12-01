fn solve(ns: Vec<i32>, off: usize) -> usize {
    (0..ns.len() - off).filter(|&i| ns[i] < ns[i + off]).count()
}

pub fn part1(input: Vec<i32>) -> usize {
    solve(input, 1)
}

pub fn part2(input: Vec<i32>) -> usize {
    solve(input, 3)
}
