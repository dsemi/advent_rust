fn solve(ns: Vec<i32>, offset: usize) -> usize {
    ns.iter().zip(&ns[offset..]).filter(|(a, b)| a < b).count()
}

pub fn part1(input: Vec<i32>) -> usize {
    solve(input, 1)
}

pub fn part2(input: Vec<i32>) -> usize {
    solve(input, 3)
}
