use crate::utils::Interval;
use bit_set::BitSet;
use scan_fmt::scan_fmt as scanf;

struct Cube {
    axis: [Interval; 3],
}

impl Cube {
    fn volume(&self) -> i64 {
        self.axis.iter().map(|i| i.len()).product()
    }

    fn intersects(&self, o: &Self) -> bool {
        self.axis
            .iter()
            .zip(o.axis.iter())
            .all(|(a, b)| a.intersects(b))
    }

    fn intersect(&self, o: &Self) -> Cube {
        Cube {
            axis: self
                .axis
                .iter()
                .zip(o.axis.iter())
                .map(|(a, b)| a.intersect(b))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

fn intersect_volume(cubes: &[Cube], bs: &[BitSet], cube: &Cube, set: &BitSet) -> i64 {
    let mut vol = cube.volume();
    for idx in set {
        let common = cube.intersect(&cubes[idx]);
        let inter = set.intersection(&bs[idx]).collect();
        vol -= intersect_volume(cubes, bs, &common, &inter);
    }
    vol
}

fn solve(input: &str, lo: i64, hi: i64) -> i64 {
    let active_cube = Cube {
        axis: [
            Interval::new(lo, hi),
            Interval::new(lo, hi),
            Interval::new(lo, hi),
        ],
    };
    let mut cubes = Vec::new();
    let mut on = Vec::new();
    for line in input.lines() {
        let (w, x0, x1, y0, y1, z0, z1) = scanf!(
            line,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let cube = Cube {
            axis: [
                Interval::new(x0, x1 + 1),
                Interval::new(y0, y1 + 1),
                Interval::new(z0, z1 + 1),
            ],
        };
        on.push(w == "on" && cube.intersects(&active_cube));
        cubes.push(cube);
    }
    let mut bs = vec![BitSet::new(); cubes.len()];
    for i in 0..cubes.len() {
        for j in 0..i {
            if cubes[i].intersects(&cubes[j]) {
                bs[j].insert(i);
            }
        }
    }
    let mut ans = 0;
    for i in 0..cubes.len() {
        if !on[i] {
            continue;
        }
        ans += intersect_volume(&cubes, &bs, &cubes[i], &bs[i]);
    }
    ans
}

pub fn part1(input: &str) -> i64 {
    solve(input, -50, 51)
}

pub fn part2(input: &str) -> i64 {
    solve(input, i64::MIN, i64::MAX)
}
