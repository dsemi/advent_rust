use crate::utils::*;
use ahash::{AHashMap, AHashSet};

pub fn part1(input: &str) -> String {
    let inp: Vec<Vec<&str>> = transpose(
        &input
            .lines()
            .map(|line| line.split(" -> ").collect())
            .collect::<Vec<_>>(),
    );
    let c = inp[1]
        .iter()
        .flat_map(|x| x.split(", "))
        .collect::<AHashSet<_>>();
    let s = inp[0]
        .iter()
        .map(|x| x.split_whitespace().next().unwrap())
        .collect::<AHashSet<_>>();
    (&s - &c).into_iter().next().map(|x| x.to_string()).unwrap()
}

struct Node {
    weight: i64,
    children: Vec<String>,
}

fn find_imbalance(m: &AHashMap<String, Node>, curr: &str) -> (i64, bool) {
    let node = &m[curr];
    if node.children.is_empty() {
        return (node.weight, false);
    }

    let recs = node
        .children
        .iter()
        .map(|x| find_imbalance(m, x))
        .collect::<Vec<_>>();
    for r in &recs {
        if r.1 {
            return *r;
        }
    }
    let wts = recs.into_iter().map(|x| x.0).collect::<Vec<_>>();
    let count = wts.iter().counts();
    if count.len() == 1 {
        return (node.weight + wts.iter().sum::<i64>(), false);
    }

    let mut cts = count.into_iter().collect::<Vec<_>>();
    cts.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let anomaly = *cts[cts.len() - 1].0;
    let expected = *cts[0].0;
    for (i, v) in wts.into_iter().enumerate() {
        if v == anomaly {
            let ans = expected - anomaly + m[&node.children[i]].weight;
            return (ans, true);
        }
    }
    panic!("Could not find imbalance")
}

pub fn part2(input: &str) -> i64 {
    let m = input
        .lines()
        .map(|line| {
            let pts = line.split(" -> ").collect::<Vec<_>>();
            let (n, w) = pts[0].split_once(" (").unwrap();
            (
                n.to_string(),
                Node {
                    weight: w[..w.len() - 1].parse().unwrap(),
                    children: if pts.len() > 1 {
                        pts[1].split(", ").map(|x| x.to_string()).collect()
                    } else {
                        vec![]
                    },
                },
            )
        })
        .collect();
    let root = part1(input);
    find_imbalance(&m, &root).0
}
