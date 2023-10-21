use itertools::Itertools;

fn solve(input: &str, nchars: usize) -> Option<usize> {
    (nchars..=input.len()).find(|&i| input[i - nchars..i].chars().all_unique())
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part2(input: &str) -> Option<usize> {
    solve(input, 14)
}
