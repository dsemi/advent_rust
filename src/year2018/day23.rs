use crate::utils::Coord3;
use scan_fmt::scan_fmt as scanf;
use std::cmp::{max, min};

struct Nanobot {
    pos: Coord3<i64>,
    radius: i64,
}

impl Nanobot {
    fn in_range(&self, coord: Coord3<i64>) -> bool {
        let p = self.pos - coord;
        p.x.abs() + p.y.abs() + p.z.abs() <= self.radius
    }

    fn div(&self, n: i64) -> Self {
        Nanobot {
            pos: self.pos.div(n),
            radius: self.radius / n,
        }
    }
}

fn parse_nanobots(input: &str) -> Vec<Nanobot> {
    input
        .lines()
        .map(|line| {
            let (x, y, z, r) = scanf!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, i64).unwrap();
            Nanobot {
                pos: Coord3::new(x, y, z),
                radius: r,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let ns = parse_nanobots(input);
    let max_bot = ns.iter().max_by_key(|n| n.radius).unwrap();
    ns.iter().filter(|n| max_bot.in_range(n.pos)).count()
}

pub fn part2(input: &str) -> i64 {
    let ns = parse_nanobots(input);
    let mut n = 10_000_000;
    let mut min_coord = ns
        .iter()
        .map(|n| n.pos)
        .reduce(|a, b| Coord3::new(min(a.x, b.x), min(a.y, b.y), min(a.z, b.z)))
        .unwrap()
        .div(n);
    let mut max_coord = ns
        .iter()
        .map(|n| n.pos)
        .reduce(|a, b| Coord3::new(max(a.x, b.x), max(a.y, b.y), max(a.z, b.z)))
        .unwrap()
        .div(n);
    let mut coord = Coord3::new(0, 0, 0);
    while n != 0 {
        let ns2 = ns.iter().map(|b| b.div(n)).collect::<Vec<_>>();
        let ns2r = &ns2;
        coord = (min_coord.x..=max_coord.x)
            .flat_map(|x| {
                (min_coord.y..=max_coord.y).flat_map(move |y| {
                    (min_coord.z..=max_coord.z).map(move |z| {
                        let p = Coord3::new(x, y, z);
                        let cnt = ns2r.iter().filter(|b| b.in_range(p)).count();
                        (cnt, p)
                    })
                })
            })
            .reduce(|(ac, ap), (bc, bp)| {
                if (bc, -bp) > (ac, -ap) {
                    (bc, bp)
                } else {
                    (ac, ap)
                }
            })
            .unwrap()
            .1;
        min_coord = (coord - Coord3::new(1, 1, 1)).scale(10);
        max_coord = (coord + Coord3::new(1, 1, 1)).scale(10);
        n /= 10;
    }
    coord.x + coord.y + coord.z
}
