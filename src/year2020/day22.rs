use ahash::AHashSet;
use std::collections::VecDeque;

fn parse_game(s: &str) -> (VecDeque<i64>, VecDeque<i64>) {
    let parts: Vec<VecDeque<i64>> = s
        .split("\n\n")
        .map(|p| p.lines().skip(1).map(|x| x.parse().unwrap()).collect())
        .collect();
    (parts[0].clone(), parts[1].clone())
}

fn play(mut a_s: VecDeque<i64>, mut b_s: VecDeque<i64>, p2: bool, sub: bool) -> (i64, bool) {
    let mut s = AHashSet::new();
    if sub && a_s.iter().max().unwrap() > b_s.iter().max().unwrap() {
        return (0, true);
    }
    while !a_s.is_empty() && !b_s.is_empty() {
        if p2 {
            let key: (VecDeque<i64>, VecDeque<i64>) = (a_s.clone(), b_s.clone());
            if !s.insert(key) {
                return (0, true);
            }
        }
        let a = a_s.pop_front().unwrap();
        let b = b_s.pop_front().unwrap();
        let p1_win = if p2 && a <= a_s.len() as i64 && b <= b_s.len() as i64 {
            play(
                a_s.iter().take(a as usize).copied().collect(),
                b_s.iter().take(b as usize).copied().collect(),
                p2,
                true,
            )
            .1
        } else {
            a > b
        };
        if p1_win {
            a_s.extend([a, b].iter());
        } else {
            b_s.extend([b, a].iter());
        }
    }
    let win = b_s.is_empty();
    let x = if a_s.is_empty() { b_s } else { a_s };
    (
        x.into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i as i64 + 1) * x)
            .sum(),
        win,
    )
}

pub fn part1(input: &str) -> i64 {
    let (a, b) = parse_game(input);
    play(a, b, false, false).0
}

pub fn part2(input: &str) -> i64 {
    let (a, b) = parse_game(input);
    play(a, b, true, false).0
}
