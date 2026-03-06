fn seat_ids(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.lines().map(|line| {
        line.bytes().map(|b| ((b & 0b100) >> 2) ^ 1).fold(0, |acc, d| (acc << 1) | d as i64)
    })
}

pub fn part1(input: &str) -> Option<i64> {
    seat_ids(input).max()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut ids: Vec<_> = seat_ids(input).collect();
    ids.sort_unstable();
    ids.array_windows().find_map(|[a, b]| (a + 2 == *b).then_some(a + 1))
}
