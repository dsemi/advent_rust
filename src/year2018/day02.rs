use counter::Counter;

pub fn part1(input: &str) -> usize {
    let counts = input
        .lines()
        .map(|line| line.chars().collect::<Counter<_>>())
        .collect::<Vec<_>>();
    let (mut twos, mut threes) = (0, 0);
    for tbl in counts {
        if tbl.values().any(|v| *v == 2) {
            twos += 1;
        }
        if tbl.values().any(|v| *v == 3) {
            threes += 1;
        }
    }
    twos * threes
}

pub fn part2(input: &str) -> Option<String> {
    let ids = input.lines().collect::<Vec<_>>();
    (0..ids.len())
        .flat_map(|a| {
            std::iter::repeat(a)
                .zip(a + 1..ids.len())
                .map(|(i, j)| (ids[i], ids[j]))
        })
        .filter_map(|(b1, b2)| {
            let mut diff = 0;
            for (a, b) in b1.chars().zip(b2.chars()) {
                if a != b {
                    diff += 1;
                }
                if diff > 1 {
                    return None;
                }
            }
            Some(
                b1.chars()
                    .zip(b2.chars())
                    .filter_map(|(a, b)| (a == b).then(|| a))
                    .collect(),
            )
        })
        .next()
}
