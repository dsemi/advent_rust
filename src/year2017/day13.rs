fn parse_firewall(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().map(|line| {
        let (a, b) = line.split_once(": ").unwrap();
        (a.parse().unwrap(), 2 * b.parse::<i64>().unwrap() - 2)
    })
}

pub fn part1(input: &str) -> i64 {
    parse_firewall(input)
        .filter_map(|(a, b)| (a % b == 0).then_some(a * (b + 2) / 2))
        .sum()
}

pub fn part2(input: &str) -> Option<i64> {
    let scrs = parse_firewall(input).collect::<Vec<_>>();
    (0..).find(|i| scrs.iter().all(|(a, b)| (a + i) % b != 0))
}
