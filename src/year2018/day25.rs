use ahash::AHashSet;
use std::cmp::Ordering::*;

struct Node {
    pt: (i64, i64, i64, i64),
    parent: usize,
    rank: usize,
}

fn parse_points(input: &str) -> Vec<Node> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let ns = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            Node {
                pt: (ns[0], ns[1], ns[2], ns[3]),
                parent: i,
                rank: 0,
            }
        })
        .collect()
}

fn dist(a: &Node, b: &Node) -> i64 {
    let (w0, x0, y0, z0) = a.pt;
    let (w1, x1, y1, z1) = b.pt;
    (w0 - w1).abs() + (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn find(points: &[Node], mut k: usize) -> usize {
    while k != points[k].parent {
        k = points[k].parent
    }
    k
}

fn union(points: &mut [Node], x: usize, y: usize) {
    let x_root = find(points, x);
    let y_root = find(points, y);
    if x_root == y_root {
        return;
    }
    match points[x_root].rank.cmp(&points[y_root].rank) {
        Less => points[x_root].parent = y_root,
        Greater => points[y_root].parent = x_root,
        Equal => {
            points[y_root].parent = x_root;
            points[x_root].rank += 1;
        }
    }
}

fn constellations(mut pts: Vec<Node>) -> usize {
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            if dist(&pts[i], &pts[j]) <= 3 {
                union(&mut pts, i, j);
            }
        }
    }
    (0..pts.len())
        .map(|p| find(&pts, p))
        .collect::<AHashSet<_>>()
        .len()
}

pub fn part1(input: &str) -> usize {
    constellations(parse_points(input))
}

pub fn part2(_input: &str) -> String {
    " ".to_string()
}
