fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn valid(sides: &[u32]) -> bool {
    *sides.iter().max().unwrap() < sides.iter().sum::<u32>() - *sides.iter().max().unwrap()
}

pub fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(|v| valid(v)).count()
}

pub fn part2(input: &str) -> usize {
    let ts = parse(input);
    let mut v: Vec<Vec<u32>> = Vec::new();
    for i in 0..ts[0].len() {
        v.push(ts.iter().map(|v| v[i]).collect());
    }
    v.into_iter()
        .map(|row| {
            row.chunks(3)
                .filter(|chunk| valid(chunk))
                .count()
        })
        .sum()
}
