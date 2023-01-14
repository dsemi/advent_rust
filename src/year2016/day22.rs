use crate::utils::*;
use ahash::AHashMap;
use regex::Regex;
use std::cmp::max;

#[derive(Clone)]
struct Node {
    coord: C<i32>,
    used: i64,
    avail: i64,
}

fn parse_nodes(input: &str) -> Vec<Node> {
    let re =
        Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    input
        .lines()
        .skip(2)
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Node {
                coord: C(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                used: cap[4].parse().unwrap(),
                avail: cap[5].parse().unwrap(),
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let nodes = parse_nodes(input);
    (0..nodes.len())
        .map(|i| {
            (i + 1..nodes.len())
                .filter(|&j| {
                    nodes[i].used > 0 && nodes[i].used < nodes[j].avail
                        || nodes[j].used > 0 && nodes[j].used < nodes[i].avail
                })
                .count()
        })
        .sum()
}

fn neighbors(grid: &AHashMap<C<i32>, Node>, st: &(C<i32>, C<i32>)) -> Vec<(C<i32>, C<i32>)> {
    vec![C(0, 1), C(0, -1), C(1, 0), C(-1, 0)]
        .into_iter()
        .filter_map(move |d| {
            let o2 = st.0 + d;
            (grid.contains_key(&o2) && grid[&o2].used <= 100)
                .then(|| (o2, if o2 == st.1 { st.0 } else { st.1 }))
        })
        .collect()
}

pub fn part2(input: &str) -> usize {
    let nodes = parse_nodes(input);
    let mut grid = AHashMap::new();
    let mut opn = C(0, 0);
    let mut mx = C(0, 0);
    for node in nodes {
        grid.insert(node.coord, node.clone());
        if node.used == 0 {
            opn = node.coord;
        }
        mx = max(mx, node.coord);
    }
    for (d, v) in bfs((opn, C(mx.0, 0)), |st| neighbors(&grid, st)) {
        if v.1 == C(0, 0) {
            return d;
        }
    }
    unreachable!()
}
