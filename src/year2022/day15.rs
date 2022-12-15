use crate::utils::*;
use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;

struct Scanner {
    pos: Coord<i64>,
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
            pos: Coord::new(sx, sy),
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
        let diff = sensor.dist - (sensor.pos.y - Y).abs();
        if diff < 0 {
            continue;
        }
        intervals.push(Interval::new(sensor.pos.x - diff, sensor.pos.x + diff + 1));
    }
    intervals.sort_unstable_by_key(|i| i.lo);
    let interval = intervals.iter().fold(intervals[0], |a, b| a.union(b));
    interval.len()
        - bs.into_iter()
            .filter(|&b| b >= interval.lo && b < interval.hi)
            .count() as i64
}

struct Line {
    s: Coord<i64>,
    e: Coord<i64>,
}

impl Line {
    fn new(s: Coord<i64>, e: Coord<i64>) -> Self {
        Self { s, e }
    }

    fn intersect(&self, o: &Self) -> Option<Coord<i64>> {
        (self.s.x <= o.e.x && o.s.x <= self.e.x && self.s.y <= o.s.y && o.e.y <= self.e.y).then(
            || {
                let (p1, p2) = (o.s.x + o.s.y, self.s.x - self.s.y);
                Coord::new((p1 + p2) / 2, (p1 - p2) / 2)
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
            Coord::new(s.pos.x - s.dist - 1, s.pos.y),
            Coord::new(s.pos.x, s.pos.y + s.dist + 1),
        ));
        urs.push(Line::new(
            Coord::new(s.pos.x, s.pos.y - s.dist - 1),
            Coord::new(s.pos.x + s.dist + 1, s.pos.y),
        ));
        drs.push(Line::new(
            Coord::new(s.pos.x, s.pos.y + s.dist + 1),
            Coord::new(s.pos.x + s.dist + 1, s.pos.y),
        ));
        drs.push(Line::new(
            Coord::new(s.pos.x - s.dist - 1, s.pos.y),
            Coord::new(s.pos.x, s.pos.y - s.dist - 1),
        ));
    }
    for a in urs.iter() {
        for b in drs.iter() {
            let pos = a.intersect(b).unwrap_or_else(|| Coord::new(-1, -1));
            if pos.x < 0 || pos.x > 4000000 || pos.y < 0 || pos.y > 4000000 {
                continue;
            }
            if !sensors.iter().any(|it| dist(&pos, &it.pos) <= it.dist) {
                return 4000000 * pos.x + pos.y;
            }
        }
    }
    unreachable!()
}
