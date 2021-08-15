use ahash::AHashMap;
use regex::Regex;
use std::cmp::max;

use crate::utils::*;

#[derive(Clone)]
struct Node {
    coord: Coord<i32>,
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
                coord: Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
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

fn neighbors(
    grid: &AHashMap<Coord<i32>, Node>,
    st: &(Coord<i32>, Coord<i32>),
) -> Vec<(Coord<i32>, Coord<i32>)> {
    vec![
        Coord::new(0, 1),
        Coord::new(0, -1),
        Coord::new(1, 0),
        Coord::new(-1, 0),
    ]
    .into_iter()
    .filter_map(move |d| {
        let o2 = st.0 + d;
        (grid.contains_key(&o2) && grid[&o2].used <= 100)
            .then(|| (o2, if o2 == st.1 { st.0 } else { st.1 }))
    })
    .collect()
}

pub fn part2(input: &str) -> Option<usize> {
    let nodes = parse_nodes(input);
    let mut grid = AHashMap::new();
    let mut opn = Coord::new(0, 0);
    let mut mx = Coord::new(0, 0);
    for node in nodes {
        grid.insert(node.coord, node.clone());
        if node.used == 0 {
            opn = node.coord;
        }
        mx = max(mx, node.coord);
    }
    let mut x = bfs((opn, Coord::new(mx.x, 0)), |st| neighbors(&grid, st))
        .filter_map(|(d, v)| (v.1 == Coord::new(0, 0)).then(|| d));
    x.next()
}
