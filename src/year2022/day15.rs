use crate::utils::*;
use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;

struct Scanner {
    pos: C<i64>,
    dist: i64,
}

fn parse_scanners(input: &str) -> (Vec<Scanner>, AHashSet<i64>) {
    let mut scanners = Vec::new();
    let mut beacon_xs = AHashSet::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = scanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let dist = (sx - bx).abs() + (sy - by).abs();
        scanners.push(Scanner {
            pos: C(sx, sy),
            dist,
        });
        if by == 2000000 {
            beacon_xs.insert(bx);
        }
    }
    (scanners, beacon_xs)
}

pub fn part1(input: &str) -> i64 {
    let (sensors, bs) = parse_scanners(input);
    const Y: i64 = 2000000;
    let mut intervals = Vec::new();
    for sensor in sensors {
        let diff = sensor.dist - (sensor.pos.1 - Y).abs();
        if diff < 0 {
            continue;
        }
        intervals.push(Interval::new(sensor.pos.0 - diff, sensor.pos.0 + diff + 1));
    }
    intervals.sort_unstable_by_key(|i| i.lo);
    let interval = intervals.iter().fold(intervals[0], |a, b| a.union(b));
    interval.len()
        - bs.into_iter()
            .filter(|&b| b >= interval.lo && b < interval.hi)
            .count() as i64
}

struct Line {
    s: C<i64>,
    e: C<i64>,
}

impl Line {
    fn new(s: C<i64>, e: C<i64>) -> Self {
        Self { s, e }
    }

    fn intersect(&self, o: &Self) -> Option<C<i64>> {
        (self.s.0 <= o.e.0 && o.s.0 <= self.e.0 && self.s.1 <= o.s.1 && o.e.1 <= self.e.1).then(
            || {
                let (p1, p2) = (o.s.0 + o.s.1, self.s.0 - self.s.1);
                C((p1 + p2) / 2, (p1 - p2) / 2)
            },
        )
    }
}

pub fn part2(input: &str) -> i64 {
    let sensors = parse_scanners(input).0;
    let mut urs = Vec::new();
    let mut drs = Vec::new();
    for s in sensors.iter() {
        urs.push(Line::new(
            C(s.pos.0 - s.dist - 1, s.pos.1),
            C(s.pos.0, s.pos.1 + s.dist + 1),
        ));
        urs.push(Line::new(
            C(s.pos.0, s.pos.1 - s.dist - 1),
            C(s.pos.0 + s.dist + 1, s.pos.1),
        ));
        drs.push(Line::new(
            C(s.pos.0, s.pos.1 + s.dist + 1),
            C(s.pos.0 + s.dist + 1, s.pos.1),
        ));
        drs.push(Line::new(
            C(s.pos.0 - s.dist - 1, s.pos.1),
            C(s.pos.0, s.pos.1 - s.dist - 1),
        ));
    }
    for a in urs.iter() {
        for b in drs.iter() {
            let pos = a.intersect(b).unwrap_or(C(-1, -1));
            if pos.0 < 0 || pos.0 > 4000000 || pos.1 < 0 || pos.1 > 4000000 {
                continue;
            }
            if !sensors.iter().any(|it| pos.dist(&it.pos) <= it.dist) {
                return 4000000 * pos.0 + pos.1;
            }
        }
    }
    unreachable!()
}
