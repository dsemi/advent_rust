fn seat_ids(s: &str) -> Vec<i64> {
    s.chars()
        .map(|x| match x {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            c => c,
        })
        .collect::<String>()
        .lines()
        .map(|line| i64::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn part1(input: &str) -> Option<i64> {
    seat_ids(input).into_iter().max()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut ids = seat_ids(input);
    ids.sort_unstable();
    ids.windows(2)
        .find_map(|wind| (wind[0] + 2 == wind[1]).then_some(wind[0] + 1))
}
