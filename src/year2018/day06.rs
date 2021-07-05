use counter::Counter;
use itertools::Itertools;
use std::cmp::{max, min};

use crate::utils::*;

fn parse_coords(input: &str) -> Vec<Coord<i32>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(", ").unwrap();
            Coord::new(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn all_within(xs: &Vec<Coord<i32>>, buffer: i32) -> impl Iterator<Item = Coord<i32>> {
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for x in xs {
        x0 = min(x0, x.x);
        y0 = min(y0, x.y);
        x1 = max(x1, x.x);
        y1 = max(y1, x.y);
    }
    x0 -= buffer;
    y0 -= buffer;
    x1 += buffer;
    y1 += buffer;
    (x0..x1 + 1).flat_map(move |x| (y0..y1 + 1).map(move |y| Coord::new(x, y)))
}

pub fn part1(input: &str) -> Option<usize> {
    let coords = parse_coords(input);
    let mut ns = Vec::new();
    for i in vec![0, 10] {
        let t = all_within(&coords, i)
            .filter_map(|coord| {
                let dists = coords
                    .iter()
                    .map(|x| (dist(&coord, x), x))
                    .collect::<Vec<_>>();
                let d = dists.iter().min().unwrap();
                (dists.iter().filter(|x| x.0 == d.0).count() == 1).then(|| d.1)
            })
            .collect::<Counter<_>>();
        ns.push(
            t.into_iter()
                .sorted()
                .map(|x| x.1)
                .copied()
                .collect::<Vec<_>>(),
        );
    }
    ns[0]
        .iter()
        .zip(ns[1].iter())
        .map(|(a, b)| if a == b { *a } else { 0 })
        .max()
}

pub fn part2(input: &str) -> usize {
    let n = 10_000;
    let coords = parse_coords(input);
    all_within(&coords, (n / coords.len()) as i32)
        .filter(|x| coords.iter().map(|y| dist(&x, y)).sum::<i32>() < n as i32)
        .count()
}
