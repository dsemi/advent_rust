use num::integer::gcd;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::BTreeMap;

use crate::utils::*;

fn parse_coords(input: &str) -> Vec<Coord<i32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, v)| v == '#')
                .map(|(x, _)| Coord::new(x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[derive(Eq, PartialEq)]
struct Angle {
    x: i32,
    y: i32,
}

impl Angle {
    fn new(a: &Coord<i32>, b: &Coord<i32>) -> Self {
        let (x, y) = (b.x - a.x, b.y - a.y);
        let gcd = gcd(x.abs(), y.abs());
        Self {
            x: x / gcd,
            y: y / gcd,
        }
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x >= 0 && other.x < 0 {
            Less
        } else if self.x < 0 && other.x >= 0 {
            Greater
        } else if self.x == 0 && other.x == 0 {
            self.y.cmp(&other.y)
        } else {
            let det = self.x * (-other.y) - (-self.y) * other.x;
            det.cmp(&0)
        }
    }
}

fn visibilities(pt: &Coord<i32>, pts: &[Coord<i32>]) -> Vec<Vec<Coord<i32>>> {
    let mut m: BTreeMap<Angle, Vec<Coord<i32>>> = BTreeMap::new();
    for p in pts.iter() {
        if p != pt {
            let e = m.entry(Angle::new(pt, p)).or_insert_with(Vec::new);
            let idx = e
                .binary_search_by_key(&dist(pt, p), |x| dist(pt, x))
                .collapse();
            e.insert(idx, *p);
        }
    }
    m.into_values().collect()
}

fn max_detected(asts: Vec<Coord<i32>>) -> Vec<Vec<Coord<i32>>> {
    asts.iter()
        .map(|ast| visibilities(ast, &asts))
        .max_by_key(|x| x.len())
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    max_detected(parse_coords(input)).len()
}

pub fn part2(input: &str) -> Option<i32> {
    max_detected(parse_coords(input))
        .into_iter()
        .map(|x| x.into_iter())
        .cycle()
        .filter_map(|mut pts| pts.next().map(|c| 100 * c.x + c.y))
        .nth(199)
}
