use ahash::AHashMap;
use itertools::Itertools;

fn parse_happiness(input: &str) -> Vec<Vec<i32>> {
    let mut d = AHashMap::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l = (input.lines().count() as f32).sqrt() as usize + 1;
    for _ in 0..l {
        result.push(vec![0; l]);
    }
    let mut key = 0;
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let (p1, n, p2) = (
            parts[0],
            if parts[2] == "gain" {
                parts[3].parse::<i32>().unwrap()
            } else {
                -parts[3].parse::<i32>().unwrap()
            },
            &parts[10][..parts[10].len() - 1],
        );
        d.entry(p1.to_string()).or_insert_with(|| {
            key += 1;
            key - 1
        });
        d.entry(p2.to_string()).or_insert_with(|| {
            key += 1;
            key - 1
        });
        result[d[p1]][d[p2]] = n;
    }
    result
}

fn max_happiness(d: Vec<Vec<i32>>) -> Option<i32> {
    (0..d.len())
        .permutations(d.len())
        .map(|mut perm| {
            perm.push(perm[0]);
            perm.windows(2).map(|p| d[p[0]][p[1]] + d[p[1]][p[0]]).sum()
        })
        .max()
}

pub fn part1(input: &str) -> Option<i32> {
    max_happiness(parse_happiness(input))
}

pub fn part2(input: &str) -> Option<i32> {
    let mut d = parse_happiness(input);
    for i in 0..d.len() {
        d[i].push(0);
    }
    d.push(vec![0; d[0].len()]);
    max_happiness(d)
}
