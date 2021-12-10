use counter::Counter;

const FREQS: &[usize] = &[42, 17, 34, 39, 30, 37, 41, 25, 49, 45];

fn parse(input: &str) -> impl Iterator<Item = Vec<usize>> + '_ {
    input.lines().map(|line| {
        let (key, ns) = line.split_once(" | ").unwrap();
        let hist = key.replace(" ", "").chars().collect::<Counter<_>>();
        ns.split_whitespace()
            .map(|n| {
                let x: usize = n.chars().map(|d| hist[&d]).sum();
                FREQS.iter().position(|&y| y == x).unwrap()
            })
            .collect()
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .flat_map(|ns| ns.into_iter().filter(|&n| [1, 4, 7, 8].contains(&n)))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .map(|ns| ns.into_iter().fold(0, |a, b| a * 10 + b))
        .sum()
}
