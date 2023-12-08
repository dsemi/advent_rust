use crate::utils::parsers2::*;
use crate::utils::*;
use ahash::AHashMap;
use std::cmp::max;

#[derive(Clone)]
struct Node {
    coord: C<i32>,
    used: i64,
    avail: i64,
}

fn node(i: &mut &str) -> PResult<Node> {
    "/dev/grid/node-x".parse_next(i)?;
    let coord = sep_tuple2(i32, "-y").output_into().parse_next(i)?;
    delimited(space1, i64, 'T').parse_next(i)?;
    let used = delimited(space1, i64, 'T').parse_next(i)?;
    let avail = delimited(space1, i64, 'T').parse_next(i)?;
    delimited(space1, i64, '%').parse_next(i)?;
    Ok(Node { coord, used, avail })
}

fn parse_nodes(input: &str) -> Vec<Node> {
    input.lines().skip(2).map(|line| node.read(line)).collect()
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

pub fn part2(input: &str) -> Option<usize> {
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
    bfs((opn, C(mx.0, 0)), move |st| neighbors(&grid, st))
        .find_map(|(d, v)| (v.1 == C(0, 0)).then_some(d))
}
