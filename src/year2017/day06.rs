use ahash::AHashMap;

fn redistribute_until_cycle(input: &str) -> (usize, usize) {
    let mut ns: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let len = ns.len();
    let mut m: AHashMap<Vec<usize>, usize> = AHashMap::new();
    for c in 0.. {
        if m.contains_key(&ns) {
            return (c, c - m[&ns]);
        }
        m.insert(ns.clone(), c);
        let (j, val) = ns
            .iter()
            .copied()
            .enumerate()
            .reduce(|(i1, x1), (i2, x2)| if x2 > x1 { (i2, x2) } else { (i1, x1) })
            .unwrap();
        ns[j] = 0;
        for k in j + 1..=j + val {
            ns[k % len] += 1;
        }
    }
    (0, 0)
}

pub fn part1(input: &str) -> usize {
    redistribute_until_cycle(input).0
}

pub fn part2(input: &str) -> usize {
    redistribute_until_cycle(input).1
}
