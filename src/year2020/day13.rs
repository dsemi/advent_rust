use crate::utils::chinese_remainder;

fn parse_buses(s: &str) -> (i64, Vec<(i64, i64)>) {
    let ls: Vec<&str> = s.lines().collect();
    let t = ls[0].parse().unwrap();
    (
        t,
        ls[1]
            .split(',')
            .enumerate()
            .filter(|&(_, x)| x != "x")
            .map(|(i, x)| (-(i as i64), x.parse().unwrap()))
            .collect(),
    )
}

pub fn part1(input: &str) -> i64 {
    let (t, buses) = parse_buses(input);
    let (a, b) = buses.iter().map(|(_, b)| (b, b - t % b)).min_by_key(|x| x.1).unwrap();
    a * b
}

pub fn part2(input: &str) -> i64 {
    chinese_remainder(parse_buses(input).1)
}
