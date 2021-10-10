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
    for i in 0..ids.len() {
        if ids[i] + 2 == ids[i + 1] {
            return Some(ids[i] + 1);
        }
    }
    None
}
