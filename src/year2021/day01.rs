fn solve(ns: Vec<i32>, off: usize) -> usize {
    ns.iter().zip(&ns[off..]).filter(|(a, b)| a < b).count()
}

pub fn part1(input: Vec<i32>) -> usize {
    solve(input, 1)
}

pub fn part2(input: Vec<i32>) -> usize {
    solve(input, 3)
}
