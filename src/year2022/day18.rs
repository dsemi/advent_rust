use crate::utils::*;
use ahash::AHashSet;

fn adj(c: &Coord3<i32>) -> Vec<Coord3<i32>> {
    vec![
        Coord3::new(c.x + 1, c.y, c.z),
        Coord3::new(c.x - 1, c.y, c.z),
        Coord3::new(c.x, c.y + 1, c.z),
        Coord3::new(c.x, c.y - 1, c.z),
        Coord3::new(c.x, c.y, c.z + 1),
        Coord3::new(c.x, c.y, c.z - 1),
    ]
}

fn cubes(input: &str) -> AHashSet<Coord3<i32>> {
    input
        .lines()
        .map(|l| {
            let pts = l.split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
            Coord3::new(pts[0], pts[1], pts[2])
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
    let mut lo = Coord3::new(i32::MAX, i32::MAX, i32::MAX);
    let mut hi = Coord3::new(i32::MIN, i32::MIN, i32::MIN);
    for c in lava.iter() {
        lo = lo.smol(&(*c - Coord3::new(1, 1, 1)));
        hi = hi.swol(&(*c + Coord3::new(1, 1, 1)));
    }
    let air = bfs_m([lo, hi], |pos| {
        adj(pos).into_iter().filter_map(|p| {
            ((lo.x..=hi.x).contains(&p.x)
                && (lo.y..=hi.y).contains(&p.y)
                && (lo.z..=hi.z).contains(&p.z)
                && !lava.contains(&p))
            .then(|| p)
        })
    })
    .map(|(_, p)| p)
    .collect::<AHashSet<_>>();
    lava.iter()
        .flat_map(adj)
        .filter(|a| air.contains(a))
        .count()
}
