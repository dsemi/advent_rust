pub fn part1(input: &str) -> String {
    const IN: &str = "=-012";
    const OUT: [char; 5] = ['0', '1', '2', '=', '-'];
    let d: ahash::AHashMap<char, i64> =
        IN.chars().enumerate().map(|(i, c)| (c, i as i64)).collect();
    let n: i64 = input
        .lines()
        .map(|l| l.chars().map(|c| d[&c] - 2).fold(0, |a, b| a * 5 + b))
        .sum();
    let ds: Vec<i64> =
        itertools::unfold(n, |n| (*n > 0).then(|| std::mem::replace(n, (*n + 2) / 5))).collect();
    ds.iter().rev().map(|n| OUT[(n % 5) as usize]).collect()
}

pub fn part2(_input: &str) -> String {
    " ".to_owned()
}
