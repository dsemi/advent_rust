use crate::utils::parsers::*;
use crate::utils::*;
use itertools::Itertools;

struct Scanner {
    pos: C<i64>,
    dist: i64,
}

fn sensor(i: &mut &str) -> Result<(i64, i64, i64, i64)> {
    let (_, sx, _, sy) = ("Sensor at x=", i64, ", y=", i64).parse_next(i)?;
    let (_, bx, _, by) = (": closest beacon is at x=", i64, ", y=", i64).parse_next(i)?;
    Ok((sx, sy, bx, by))
}

fn parse_scanners(input: &str) -> (Vec<Scanner>, Vec<C<i64>>) {
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = sensor.read(line);
        let dist = (sx - bx).abs() + (sy - by).abs();
        scanners.push(Scanner { pos: C(sx, sy), dist });
        beacons.push(C(bx, by));
    }
    (scanners, beacons)
}

fn compress(ints: Vec<Interval<i64>>) -> Vec<Interval<i64>> {
    let mut ints = ints.into_iter();
    let mut comp = Vec::new();
    let mut cur = ints.next().unwrap();
    for int in ints {
        if let Some(u) = cur.union(&int) {
            cur = u;
        } else {
            comp.push(cur);
            cur = int;
        }
    }
    comp.push(cur);
    comp
}

pub fn part1(input: &str) -> i64 {
    let (sensors, beacons) = parse_scanners(input);
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
    let comp = compress(intervals);
    comp.iter().map(|i| i.len()).sum::<i64>()
        - beacons
            .into_iter()
            .filter(|&C(x, y)| y == Y && comp.iter().any(|i| i.contains(x)))
            .unique()
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
        urs.push(Line::new(C(s.pos.0 - s.dist - 1, s.pos.1), C(s.pos.0, s.pos.1 + s.dist + 1)));
        urs.push(Line::new(C(s.pos.0, s.pos.1 - s.dist - 1), C(s.pos.0 + s.dist + 1, s.pos.1)));
        drs.push(Line::new(C(s.pos.0, s.pos.1 + s.dist + 1), C(s.pos.0 + s.dist + 1, s.pos.1)));
        drs.push(Line::new(C(s.pos.0 - s.dist - 1, s.pos.1), C(s.pos.0, s.pos.1 - s.dist - 1)));
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
