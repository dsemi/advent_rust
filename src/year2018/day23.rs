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
        p.abs().sum() <= self.radius
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

#[derive(Debug)]
struct Cube {
    lo: Coord3<i64>,
    hi: Coord3<i64>,
}

impl Cube {
    fn children(&self) -> Vec<Cube> {
        let mid = (self.lo + self.hi).div(2);
        vec![
            Cube {
                lo: Coord3::new(mid.x + 1, mid.y + 1, mid.z + 1),
                hi: Coord3::new(self.hi.x, self.hi.y, self.hi.z),
            },
            Cube {
                lo: Coord3::new(mid.x + 1, mid.y + 1, self.lo.z),
                hi: Coord3::new(self.hi.x, self.hi.y, mid.z),
            },
            Cube {
                lo: Coord3::new(mid.x + 1, self.lo.y, mid.z + 1),
                hi: Coord3::new(self.hi.x, mid.y, self.hi.z),
            },
            Cube {
                lo: Coord3::new(mid.x + 1, self.lo.y, self.lo.z),
                hi: Coord3::new(self.hi.x, mid.y, mid.z),
            },
            Cube {
                lo: Coord3::new(self.lo.x, mid.y + 1, mid.z + 1),
                hi: Coord3::new(mid.x, self.hi.y, self.hi.z),
            },
            Cube {
                lo: Coord3::new(self.lo.x, mid.y + 1, self.lo.z),
                hi: Coord3::new(mid.x, self.hi.y, mid.z),
            },
            Cube {
                lo: Coord3::new(self.lo.x, self.lo.y, mid.z + 1),
                hi: Coord3::new(mid.x, mid.y, self.hi.z),
            },
            Cube {
                lo: Coord3::new(self.lo.x, self.lo.y, self.lo.z),
                hi: Coord3::new(mid.x, mid.y, mid.z),
            },
        ]
    }

    fn in_range(&self, n: &Nanobot) -> bool {
        let p = Coord3::new(
            max(self.lo.x, min(self.hi.x, n.pos.x)),
            max(self.lo.y, min(self.hi.y, n.pos.y)),
            max(self.lo.z, min(self.hi.z, n.pos.z)),
        );
        (p - n.pos).abs().sum() <= n.radius
    }
}

pub fn part2(input: &str) -> i64 {
    let ns = parse_nanobots(input);
    let mut cube = Cube {
        lo: Coord3::new(i64::MAX, i64::MAX, i64::MAX),
        hi: Coord3::new(i64::MIN, i64::MIN, i64::MIN),
    };
    for n in ns.iter() {
        cube.lo = Coord3::new(
            min(cube.lo.x, n.pos.x - n.radius),
            min(cube.lo.y, n.pos.y - n.radius),
            min(cube.lo.z, n.pos.z - n.radius),
        );
        cube.hi = Coord3::new(
            max(cube.hi.x, n.pos.x + n.radius),
            max(cube.hi.y, n.pos.y + n.radius),
            max(cube.hi.z, n.pos.z + n.radius),
        );
    }
    while cube.lo.x < cube.hi.x || cube.lo.y < cube.hi.y || cube.lo.z < cube.hi.z {
        cube = cube
            .children()
            .into_iter()
            .max_by_key(|c| ns.iter().filter(|n| c.in_range(n)).count())
            .unwrap();
    }
    cube.lo.abs().sum()
}
