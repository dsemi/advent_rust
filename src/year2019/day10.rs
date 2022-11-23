use crate::utils::*;
use num::integer::gcd;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::BTreeMap;

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

fn max_detected(input: &str) -> Vec<Vec<Coord<i32>>> {
    let asts = parse_coords(input);
    let dim = input.chars().position(|c| c == '\n').unwrap();
    let fs = fractions(dim);
    let mut reduced = [[0; 50]; 50];
    for (i, f) in fs.into_iter().enumerate() {
        let mut g = f;
        while g.y < dim as i32 && g.x < dim as i32 {
            reduced[g.y as usize][g.x as usize] = i;
            g += f;
        }
    }
    let max = asts
        .iter()
        .max_by_key(|&ast| {
            let mut visible = asts.len() - 1;
            let mut b = [[false; 2048]; 4];
            for ast2 in asts.iter() {
                if ast == ast2 {
                    continue;
                }
                let mut dx = ast.y - ast2.y;
                let mut dy = ast2.x - ast.x;
                let mut quad = 0;
                if dy > 0 && dx <= 0 {
                    quad = 1;
                    dx = std::mem::replace(&mut dy, -dx);
                } else if dx < 0 && dy <= 0 {
                    quad = 2;
                    dx = -dx;
                    dy = -dy;
                } else if dx >= 0 && dy < 0 {
                    quad = 3;
                    dy = std::mem::replace(&mut dx, -dy);
                }
                let g = reduced[dy as usize][dx as usize];
                if std::mem::replace(&mut b[quad][g], true) {
                    visible -= 1;
                }
            }
            visible
        })
        .unwrap();
    visibilities(max, &asts)
}

fn fractions(n: usize) -> Vec<Coord<i32>> {
    let mut fs = Vec::new();
    let mut stack = vec![Coord::new(1, 1)];
    let mut l = Coord::new(1, 0);
    while let Some(mut r) = stack.last().copied() {
        fs.push(l);
        while l.x + r.x < n as i32 {
            r += l;
            stack.push(r);
        }
        l = stack.pop().unwrap();
    }
    fs.push(Coord::new(1, 1));
    for i in (1..fs.len() - 1).rev() {
        fs.push(Coord::new(fs[i].y, fs[i].x));
    }
    fs
}

pub fn part1(input: &str) -> usize {
    max_detected(input).len()
}

pub fn part2(input: &str) -> Option<i32> {
    max_detected(input)
        .into_iter()
        .map(|x| x.into_iter())
        .cycle()
        .filter_map(|mut pts| pts.next().map(|c| 100 * c.x + c.y))
        .nth(199)
}
