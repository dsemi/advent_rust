fn solve(input: &str, off: usize) -> usize {
    let ns: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    (0..ns.len() - off).filter(|&i| ns[i] < ns[i + off]).count()
}

pub fn part1(input: &str) -> usize {
    solve(input, 1)
}

pub fn part2(input: &str) -> usize {
    solve(input, 3)
}
