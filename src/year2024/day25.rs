pub fn part1(input: &str) -> usize {
    let (keys, locks): (Vec<_>, _) = input
        .split("\n\n")
        .map(|s| s.bytes().filter(|&b| b != b'\n').fold(0, |acc, b| acc << 1 | (b == b'#') as u64))
        .partition(|x| x & 1 == 1);
    keys.iter().map(|key| locks.iter().filter(|&lock| key & lock == 0).count()).sum()
}

pub fn part2(_input: &str) -> &str {
    " "
}
