use itertools::Itertools;
use scan_fmt::scan_fmt as scanf;
use std::cmp::Ordering::*;

type Coord = (i64, i64, i64);

fn parse_scanners(input: &str) -> Vec<Vec<Coord>> {
    input
        .split("\n\n")
        .map(|sc| {
            sc.lines()
                .skip(1)
                .map(|line| scanf!(line, "{},{},{}", i64, i64, i64).unwrap())
                .collect()
        })
        .collect()
}

fn normalise_coord((mut x, mut y, mut z): Coord) -> Coord {
    let valsort = |lhs: &mut i64, rhs: &mut i64| {
        if lhs.abs() > rhs.abs() {
            let (olhs, orhs) = (*lhs, *rhs);
            if *rhs <= 0 {
                *rhs = olhs;
                *lhs = -orhs;
            } else {
                *rhs = -olhs;
                *lhs = orhs;
            }
        }
    };
    valsort(&mut x, &mut z);
    valsort(&mut x, &mut y);
    valsort(&mut y, &mut z);
    if x < 0 {
        x = -x;
        y = -y;
    }
    if y < 0 {
        y = -y;
        z = -z;
    }
    (x, y, z)
}

const TRANS: [&dyn Fn(Coord) -> Coord; 24] = [
    &|(x, y, z)| (x, y, z),
    &|(x, y, z)| (-y, x, z),
    &|(x, y, z)| (y, -x, z),
    &|(x, y, z)| (-x, -y, z),
    &|(x, y, z)| (x, -z, y),
    &|(x, y, z)| (z, x, y),
    &|(x, y, z)| (-z, -x, y),
    &|(x, y, z)| (-x, z, y),
    &|(x, y, z)| (x, z, -y),
    &|(x, y, z)| (-z, x, -y),
    &|(x, y, z)| (z, -x, -y),
    &|(x, y, z)| (-x, -z, -y),
    &|(x, y, z)| (x, -y, -z),
    &|(x, y, z)| (y, x, -z),
    &|(x, y, z)| (-y, -x, -z),
    &|(x, y, z)| (-x, y, -z),
    &|(x, y, z)| (z, y, -x),
    &|(x, y, z)| (-y, z, -x),
    &|(x, y, z)| (y, -z, -x),
    &|(x, y, z)| (-z, -y, -x),
    &|(x, y, z)| (-z, y, x),
    &|(x, y, z)| (-y, -z, x),
    &|(x, y, z)| (y, z, x),
    &|(x, y, z)| (z, -y, x),
];

fn find_valid_transformer(from: Coord, to: Coord) -> (usize, &'static dyn Fn(Coord) -> Coord) {
    for (idx, tran) in TRANS.iter().copied().enumerate() {
        if tran(from) == to {
            return (idx, tran);
        }
    }
    unreachable!()
}

fn solve(input: &Vec<Vec<Coord>>) -> (Vec<(usize, Coord)>, Vec<Coord>) {
    let mut norms: Vec<(usize, Vec<(Coord, Coord, Coord)>)> = input
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut norm: Vec<_> = s
                .iter()
                .flat_map(|a| {
                    s.iter().filter_map(move |b| {
                        (a != b).then(|| {
                            let norm = normalise_coord((a.0 - b.0, a.1 - b.1, a.2 - b.2));
                            (norm, *a, *b)
                        })
                    })
                })
                .collect();
            norm.sort_unstable();
            (i, norm)
        })
        .collect();
    let (psidx, mut active_norms) = norms.remove(0);
    let mut full_norms = active_norms.clone();
    let mut scanner_loc = vec![(0, (0, 0, 0))];
    let mut known_beacons = input[psidx].clone();

    let mut candidatecount = vec![];
    while !norms.is_empty() {
        active_norms.clear();
        active_norms.extend(full_norms.iter().copied());
        active_norms.sort_unstable();
        norms.retain(|(nidx, n)| {
            candidatecount.clear();
            let lhstmp = active_norms.iter().group_by(|x| x.0);
            let mut lhsiter = lhstmp
                .into_iter()
                .map(|x| x.1.collect::<Vec<_>>())
                .peekable();
            let rhstmp = n.iter().group_by(|x| x.0);
            let mut rhsiter = rhstmp
                .into_iter()
                .map(|x| x.1.collect::<Vec<_>>())
                .peekable();
            while lhsiter.peek().is_some() && rhsiter.peek().is_some() {
                let (lhs, rhs) = (lhsiter.peek().unwrap(), rhsiter.peek().unwrap());
                match lhs[0].0.cmp(&rhs[0].0) {
                    Equal => {
                        for (lhs, rhs) in lhs
                            .iter()
                            .flat_map(|lhs| rhs.iter().map(move |rhs| (lhs, rhs)))
                        {
                            let (lx, ly, lz) = (
                                lhs.2 .0 - lhs.1 .0,
                                lhs.2 .1 - lhs.1 .1,
                                lhs.2 .2 - lhs.1 .2,
                            );
                            let (rx, ry, rz) = (
                                rhs.2 .0 - rhs.1 .0,
                                rhs.2 .1 - rhs.1 .1,
                                rhs.2 .2 - rhs.1 .2,
                            );
                            let (tranid, tran) = find_valid_transformer((rx, ry, rz), (lx, ly, lz));
                            let (lsx, lsy, lsz) = lhs.1;
                            let (rsx, rsy, rsz) = tran(rhs.1);
                            let offset = (lsx - rsx, lsy - rsy, lsz - rsz);
                            candidatecount.push((offset, tranid, tran));
                        }
                        lhsiter.next();
                        rhsiter.next();
                    }
                    Less => {
                        lhsiter.next();
                    }
                    Greater => {
                        rhsiter.next();
                    }
                }
            }
            candidatecount.sort_unstable_by_key(|x| (x.0, x.1));
            let transinfo = candidatecount
                .iter()
                .group_by(|x| (x.0, x.1))
                .into_iter()
                .map(|x| x.1.collect::<Vec<_>>())
                .filter(|candidate| candidate.len() >= 6)
                .max_by_key(|candidate| candidate.len());
            if transinfo.is_none() {
                return true;
            }
            let candidate = transinfo.unwrap();
            let ((ox, oy, oz), _, tran) = candidate[0];
            full_norms.extend(n.iter().map(|(norm, lhs, rhs)| {
                let (tx, ty, tz) = tran(*lhs);
                let lhs = (tx + ox, ty + oy, tz + oz);
                let (tx, ty, tz) = tran(*rhs);
                let rhs = (tx + ox, ty + oy, tz + oz);
                (*norm, lhs, rhs)
            }));

            known_beacons.extend(input[*nidx].iter().map(|c| {
                let (tx, ty, tz) = tran(*c);
                (tx + ox, ty + oy, tz + oz)
            }));
            scanner_loc.push((*nidx, (*ox, *oy, *oz)));
            false
        });
    }
    (scanner_loc, known_beacons)
}

pub fn part1(input: &str) -> usize {
    let (_, mut beacons) = solve(&parse_scanners(input));
    beacons.sort_unstable();
    beacons.dedup();
    beacons.len()
}

pub fn part2(input: &str) -> Option<i64> {
    let (scanners, _) = solve(&parse_scanners(input));
    scanners
        .iter()
        .flat_map(|(_, a)| {
            scanners
                .iter()
                .map(move |(_, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        })
        .max()
}
