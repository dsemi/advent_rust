use gcd::Gcd;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::BTreeMap;

type Coord = (i32, i32);

fn parse_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, v)| v == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn dist(a: &Coord, b: &Coord) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

#[derive(Eq, PartialEq)]
struct Angle {
    x: i32,
    y: i32,
}

impl Angle {
    fn new(a: &Coord, b: &Coord) -> Self {
        let (x, y) = (b.0 - a.0, b.1 - a.1);
        let gcd = (x.abs() as u32).gcd(y.abs() as u32) as i32;
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

fn visibilities(pt: &Coord, pts: &[Coord]) -> Vec<Vec<Coord>> {
    let mut m: BTreeMap<Angle, Vec<Coord>> = BTreeMap::new();
    for p in pts.iter() {
        if p != pt {
            let e = m.entry(Angle::new(pt, p)).or_insert_with(Vec::new);
            let idx = match e.binary_search_by_key(&dist(pt, p), |x| dist(pt, x)) {
                Ok(i) => i,
                Err(i) => i,
            };
            e.insert(idx, *p);
        }
    }
    m.into_values().collect()
}

fn max_detected(asts: Vec<Coord>) -> Vec<Vec<Coord>> {
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
        .filter_map(|mut pts| pts.next().map(|(a, b)| 100 * a + b))
        .nth(199)
}
