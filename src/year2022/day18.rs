use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashSet;

fn adj(&C3(x, y, z): &C3<i32>) -> Vec<C3<i32>> {
    vec![
        C3(x + 1, y, z),
        C3(x - 1, y, z),
        C3(x, y + 1, z),
        C3(x, y - 1, z),
        C3(x, y, z + 1),
        C3(x, y, z - 1),
    ]
}

fn cubes(input: &str) -> AHashSet<C3<i32>> {
    input
        .lines()
        .map(|l| {
            let pts: Vec<_> = l.split(',').map(int).collect();
            C3(pts[0], pts[1], pts[2])
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let lava = cubes(input);
    lava.iter()
        .flat_map(adj)
        .filter(|a| !lava.contains(a))
        .count()
}

pub fn part2(input: &str) -> usize {
    let lava = cubes(input);
    let mut lo = C3(i32::MAX, i32::MAX, i32::MAX);
    let mut hi = C3(i32::MIN, i32::MIN, i32::MIN);
    for c in lava.iter() {
        lo = lo.smol(*c - C3(1, 1, 1));
        hi = hi.swol(*c + C3(1, 1, 1));
    }
    let air = bfs_m([lo, hi], |pos| {
        adj(pos).into_iter().filter(|p| {
            (lo.0..=hi.0).contains(&p.0)
                && (lo.1..=hi.1).contains(&p.1)
                && (lo.2..=hi.2).contains(&p.2)
                && !lava.contains(p)
        })
    })
    .map(|(_, p)| p)
    .collect::<AHashSet<_>>();
    lava.iter()
        .flat_map(adj)
        .filter(|a| air.contains(a))
        .count()
}
