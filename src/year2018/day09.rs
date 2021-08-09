use ahash::AHashMap;
use std::collections::VecDeque;

fn parse(input: &str) -> (usize, usize) {
    scan_fmt!(
        input,
        "{} players; last marble is worth {} points",
        usize,
        usize
    )
    .unwrap()
}

fn play(n: usize, s: usize) -> Option<usize> {
    let mut m = AHashMap::new();
    let mut arr = vec![0].into_iter().collect::<VecDeque<_>>();
    for p in 1..=s {
        if p % 23 != 0 {
            arr.rotate_left(1);
            arr.push_back(p);
            continue;
        }
        arr.rotate_right(7);
        let v = arr.pop_back().unwrap();
        arr.rotate_left(1);
        let e = m.entry(p % n).or_insert(0);
        *e += p + v;
    }
    m.values().copied().max()
}

pub fn part1(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    play(a, b)
}

pub fn part2(input: &str) -> Option<usize> {
    let (a, b) = parse(input);
    play(a, b * 100)
}
