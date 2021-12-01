use ahash::AHashMap;
use std::cmp::Reverse;

fn redistribute_until_cycle(mut ns: Vec<usize>) -> (usize, usize) {
    let len = ns.len();
    let mut m: AHashMap<Vec<usize>, usize> = AHashMap::new();
    for c in 0.. {
        if m.contains_key(&ns) {
            return (c, c - m[&ns]);
        }
        m.insert(ns.clone(), c);
        let (j, &val) = ns
            .iter()
            .enumerate()
            .max_by_key(|&(i, x)| (x, Reverse(i)))
            .unwrap();
        ns[j] = 0;
        for k in j + 1..=j + val {
            ns[k % len] += 1;
        }
    }
    (0, 0)
}

pub fn part1(input: Vec<usize>) -> usize {
    redistribute_until_cycle(input).0
}

pub fn part2(input: Vec<usize>) -> usize {
    redistribute_until_cycle(input).1
}
