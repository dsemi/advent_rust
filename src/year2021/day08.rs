use counter::Counter;

fn parse(input: &str) -> impl Iterator<Item = Vec<u32>> + '_ {
    input.lines().map(|line| {
        let (key, ns) = line.split_once(" | ").unwrap();
        let hist = key.replace(" ", "").chars().collect::<Counter<_>>();
        ns.split_whitespace()
            .map(|n| match n.chars().map(|d| hist[&d]).sum() {
                42 => 0,
                17 => 1,
                34 => 2,
                39 => 3,
                30 => 4,
                37 => 5,
                41 => 6,
                25 => 7,
                49 => 8,
                45 => 9,
                _ => panic!("Bad digit"),
            })
            .collect()
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .flat_map(|ns| ns.into_iter().filter(|&n| [1, 4, 7, 8].contains(&n)))
        .count()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|ns| ns.into_iter().fold(0, |a, b| a * 10 + b))
        .sum()
}
