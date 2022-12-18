use crate::utils::*;
use ahash::AHashSet;
use std::cmp::{max, min};

fn adj(c: &Coord3<i32>) -> Vec<Coord3<i32>> {
    [
        Coord3::new(1, 0, 0),
        Coord3::new(-1, 0, 0),
        Coord3::new(0, 1, 0),
        Coord3::new(0, -1, 0),
        Coord3::new(0, 0, 1),
        Coord3::new(0, 0, -1),
    ]
    .into_iter()
    .map(|d| *c + d)
    .collect()
}

pub fn part1(input: &str) -> usize {
    let mut res = 0;
    let mut space = AHashSet::new();
    for line in input.lines() {
        let pts = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        let c = Coord3::new(pts[0], pts[1], pts[2]);
        res += 6;
        for c2 in adj(&c) {
            if space.contains(&c2) {
                res -= 2;
            }
        }
        space.insert(c);
    }
    res
}

pub fn part2(input: &str) -> usize {
    let mut res = 0;
    let mut space = AHashSet::new();
    for line in input.lines() {
        let pts = line
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        space.insert(Coord3::new(pts[0], pts[1], pts[2]));
    }
    let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
    let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
    let (mut min_z, mut max_z) = (i32::MAX, i32::MIN);
    for c in space.iter() {
        min_x = min(min_x, c.x - 1);
        max_x = max(max_x, c.x + 1);
        min_y = min(min_y, c.y - 1);
        max_y = max(max_y, c.y + 1);
        min_z = min(min_z, c.z - 1);
        max_z = max(max_z, c.z + 1);
    }
    bfs_m(
        [
            Coord3::new(min_x, min_y, min_z),
            Coord3::new(min_x, min_y, min_z),
        ],
        |pos| {
            adj(pos)
                .into_iter()
                .filter_map(|pos2| {
                    if pos2.x < min_x
                        || pos2.x > max_x
                        || pos2.y < min_y
                        || pos2.y > max_y
                        || pos2.z < min_z
                        || pos2.z > max_z
                    {
                        return None;
                    }
                    if space.contains(&pos2) {
                        res += 1;
                        return None;
                    }
                    Some(pos2)
                })
                .collect::<Vec<_>>()
        },
    )
    .count();
    res
}
