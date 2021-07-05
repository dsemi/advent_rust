use std::iter;

const PATTERN: [i64; 4] = [0, 1, 0, -1];

pub fn part1(input: &str) -> String {
    let mut ns: Vec<i64> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();
    for _ in 0..100 {
        ns = (0..ns.len())
            .map(|n| {
                PATTERN
                    .iter()
                    .flat_map(|x| iter::repeat(*x).take(n+1))
                    .cycle()
                    .skip(1)
                    .zip(ns.iter())
                    .map(|(a, b)| a * b)
                    .sum::<i64>()
                    .abs()
                    % 10
            })
            .collect();
    }
    ns[..8]
        .iter()
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect()
}

pub fn part2(input: &str) -> String {
    let offset: usize = input[..7].parse().unwrap();
    let ns: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut ds: Vec<usize> = (1..=10000).flat_map(|_| ns.iter()).copied().collect();
    assert!(offset > ds.len() / 2, "Offset is not large enough");
    ds = ds[offset..].to_vec();
    for _ in 0..100 {
        for i in (1..ds.len()).rev() {
            ds[i-1] += ds[i];
            ds[i] = ds[i] % 10;
        }
        ds[0] = ds[0] % 10;
    }
    ds[..8]
        .iter()
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect()
}
