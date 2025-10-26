use crate::utils::parsers::*;
use crate::utils::*;
use std::cmp::{max, min};

struct Nanobot {
    pos: C3<i64>,
    radius: i64,
}

impl Nanobot {
    fn in_range(&self, coord: C3<i64>) -> bool {
        let p = self.pos - coord;
        p.abs().sum() <= self.radius
    }
}

fn nanobot(i: &mut &str) -> ModalResult<Nanobot> {
    let pos = preceded("pos=<", c3(i64)).parse_next(i)?;
    let radius = preceded(">, r=", i64).parse_next(i)?;
    Ok(Nanobot { pos, radius })
}

pub fn part1(input: &str) -> usize {
    let ns = lines(nanobot).read(input);
    let max_bot = ns.iter().max_by_key(|n| n.radius).unwrap();
    ns.iter().filter(|n| max_bot.in_range(n.pos)).count()
}

#[derive(Debug)]
struct Cube {
    lo: C3<i64>,
    hi: C3<i64>,
}

impl Cube {
    fn children(&self) -> Vec<Cube> {
        let mid = (self.lo + self.hi) / 2;
        vec![
            Cube {
                lo: C3(mid.0 + 1, mid.1 + 1, mid.2 + 1),
                hi: C3(self.hi.0, self.hi.1, self.hi.2),
            },
            Cube { lo: C3(mid.0 + 1, mid.1 + 1, self.lo.2), hi: C3(self.hi.0, self.hi.1, mid.2) },
            Cube { lo: C3(mid.0 + 1, self.lo.1, mid.2 + 1), hi: C3(self.hi.0, mid.1, self.hi.2) },
            Cube { lo: C3(mid.0 + 1, self.lo.1, self.lo.2), hi: C3(self.hi.0, mid.1, mid.2) },
            Cube { lo: C3(self.lo.0, mid.1 + 1, mid.2 + 1), hi: C3(mid.0, self.hi.1, self.hi.2) },
            Cube { lo: C3(self.lo.0, mid.1 + 1, self.lo.2), hi: C3(mid.0, self.hi.1, mid.2) },
            Cube { lo: C3(self.lo.0, self.lo.1, mid.2 + 1), hi: C3(mid.0, mid.1, self.hi.2) },
            Cube { lo: C3(self.lo.0, self.lo.1, self.lo.2), hi: C3(mid.0, mid.1, mid.2) },
        ]
    }

    fn in_range(&self, n: &Nanobot) -> bool {
        let p = C3(
            max(self.lo.0, min(self.hi.0, n.pos.0)),
            max(self.lo.1, min(self.hi.1, n.pos.1)),
            max(self.lo.2, min(self.hi.2, n.pos.2)),
        );
        n.in_range(p)
    }
}

pub fn part2(input: &str) -> i64 {
    let ns = lines(nanobot).read(input);
    let mut cube =
        Cube { lo: C3(i64::MAX, i64::MAX, i64::MAX), hi: C3(i64::MIN, i64::MIN, i64::MIN) };
    for n in ns.iter() {
        cube.lo = C3(
            min(cube.lo.0, n.pos.0 - n.radius),
            min(cube.lo.1, n.pos.1 - n.radius),
            min(cube.lo.2, n.pos.2 - n.radius),
        );
        cube.hi = C3(
            max(cube.hi.0, n.pos.0 + n.radius),
            max(cube.hi.1, n.pos.1 + n.radius),
            max(cube.hi.2, n.pos.2 + n.radius),
        );
    }
    while cube.lo.0 < cube.hi.0 || cube.lo.1 < cube.hi.1 || cube.lo.2 < cube.hi.2 {
        cube = cube
            .children()
            .into_iter()
            .max_by_key(|c| ns.iter().filter(|n| c.in_range(n)).count())
            .unwrap();
    }
    cube.lo.abs().sum()
}
