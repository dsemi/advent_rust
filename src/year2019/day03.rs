use std::collections::HashMap;

use crate::utils::Coord;

fn parse_wires<'a>(input: &'a str) -> impl Iterator<Item = HashMap<Coord<i32>, i32>> + 'a {
    input.lines().map(|line| {
        let mut m = HashMap::new();
        let mut pos = Coord::new(0, 0);
        let mut steps = 0;
        for p in line.split(',') {
            let d = match &p[0..1] {
                "U" => Coord::new(0, 1),
                "D" => Coord::new(0, -1),
                "L" => Coord::new(-1, 0),
                "R" => Coord::new(1, 0),
                _ => panic!("Unknown direction: {}", p),
            };
            for _ in 0..p[1..p.len()].parse().unwrap() {
                pos += d;
                steps += 1;
                if !m.contains_key(&pos) {
                    m.insert(pos, steps);
                }
            }
        }
        m
    })
}

pub fn part1(input: &str) -> Option<i32> {
    parse_wires(input)
        .reduce(|mut a, b| {
            a.retain(|k, _v| b.contains_key(&k));
            a
        })
        .unwrap()
        .keys()
        .map(|k| k.x.abs() + k.y.abs())
        .min()
}

pub fn part2(input: &str) -> Option<i32> {
    parse_wires(input)
        .reduce(|mut a, b| {
            a.retain(|k, _v| b.contains_key(&k));
            for (k, v) in a.iter_mut() {
                *v += b[&k];
            }
            a
        })
        .unwrap()
        .values()
        .copied()
        .min()
}
