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
    for (i, b1) in ids.iter().enumerate() {
        for b2 in ids[i+1..].iter() {
            let common = b1
                .chars()
                .zip(b2.chars())
                .filter_map(|(a, b)| (a == b).then(|| a))
                .collect::<String>();
            if common.len() + 1 == b1.len() {
                return Some(common)
            }
        }
    }
    None
}
