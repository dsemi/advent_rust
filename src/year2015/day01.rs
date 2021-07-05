pub fn part1(input: &str) -> i64 {
    input.chars().map(|x| if x == '(' { 1 } else { -1 }).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    input
        .chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .position(|x| x < 0)
        .map(|x| x + 1)
}
