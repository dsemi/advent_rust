use ahash::AHashSet;

const S: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let a = line[..line.len() / 2].chars().collect::<AHashSet<_>>();
            let b = line[line.len() / 2..].chars().collect::<AHashSet<_>>();
            let ch = *a.intersection(&b).next().unwrap();
            S.chars().position(|c| c == ch).unwrap() + 1
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|pts| {
            let a = pts[0].chars().collect::<AHashSet<_>>();
            let b = pts[1].chars().collect::<AHashSet<_>>();
            let c = pts[2].chars().collect::<AHashSet<_>>();
            let ch = *a
                .intersection(&b)
                .copied()
                .collect::<AHashSet<_>>()
                .intersection(&c)
                .next()
                .unwrap();
            S.chars().position(|c| c == ch).unwrap() + 1
        })
        .sum()
}
