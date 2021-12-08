fn char_set(cs: &str) -> u32 {
    cs.as_bytes()
        .iter()
        .fold(0, |a, b| a | (1 << b - b'a') as u32)
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<u32>, Vec<u32>)> + '_ {
    input.lines().map(|line| {
        let (p1, p2) = line.split_once(" | ").unwrap();
        let mut key = p1.split_whitespace().map(char_set).collect::<Vec<_>>();
        key.sort_unstable_by_key(|n| n.count_ones());
        let ns = p2.split_whitespace().map(char_set).collect();
        (key, ns)
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .flat_map(|(_, ns)| {
            ns.into_iter()
                .filter(|&n| [2, 3, 4, 7].contains(&n.count_ones()))
        })
        .count()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(k, ns)| {
            ns.iter()
                .map(|n| {
                    match (
                        n.count_ones(),
                        (n & k[2]).count_ones(),
                        (n & k[0]).count_ones(),
                    ) {
                        (2, _, _) => 1,
                        (3, _, _) => 7,
                        (4, _, _) => 4,
                        (7, _, _) => 8,
                        (5, 2, _) => 2,
                        (5, 3, 1) => 5,
                        (5, 3, 2) => 3,
                        (6, 4, _) => 9,
                        (6, 3, 1) => 6,
                        (6, 3, 2) => 0,
                        _ => panic!("Bad digit"),
                    }
                })
                .fold(0, |a, b| a * 10 + b)
        })
        .sum()
}
