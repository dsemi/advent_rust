use crate::utils::bits;
use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;
use std::cmp::{max, min};

type Pt = [i32; 3];

fn mins(a: &Pt, b: &Pt) -> Pt {
    [min(a[0], b[0]), min(a[1], b[1]), min(a[2], b[2])]
}

fn hash(p: &Pt) -> u64 {
    ((p[0] as u64) << 42) ^ ((p[1] as u64) << 21) ^ p[2] as u64
}

struct Scanner {
    ps: Vec<Pt>,
    offset: Pt,
    min: Pt,
}

impl Scanner {
    fn add(&mut self, p: Pt) {
        self.min = mins(&self.min, &p);
        self.ps.push(p);
    }
}

fn parse_scanners(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|sc| {
            let mut scanner = Scanner {
                ps: vec![],
                offset: [0; 3],
                min: [0; 3],
            };
            for line in sc.lines().skip(1) {
                let (x, y, z) = scanf!(line, "{},{},{}", i32, i32, i32).unwrap();
                scanner.add([x, y, z]);
            }
            scanner
        })
        .collect()
}

fn can_align<const AA: usize>(a: &Scanner, b: &Scanner) -> Option<(i32, usize, bool)> {
    let mut collision = [0_u8; 4096 * 6];
    for pa in &a.ps {
        for pb in &b.ps {
            let mut base = 0;
            for mut n in [
                2048 + (pb[0] - b.min[0]) - (pa[AA] - a.min[AA]),
                (pb[0] - b.min[0]) + (pa[AA] - a.min[AA]),
                2048 + (pb[1] - b.min[1]) - (pa[AA] - a.min[AA]),
                (pb[1] - b.min[1]) + (pa[AA] - a.min[AA]),
                2048 + (pb[2] - b.min[2]) - (pa[AA] - a.min[AA]),
                (pb[2] - b.min[2]) + (pa[AA] - a.min[AA]),
            ] {
                let idx = base + n as usize;
                collision[idx] += 1;
                if collision[idx] == 12 {
                    let ori = idx / 4096;
                    let axis = ori / 2;
                    let negate = ori % 2 == 1;
                    n += b.min[axis];
                    if negate {
                        n += a.min[AA]
                    } else {
                        n -= a.min[AA] + 2048;
                    }

                    return Some((n, axis, negate));
                }
                base += 4096;
            }
        }
    }
    None
}

fn align<const AA: usize>(b: &mut Scanner, n: i32, axis: usize, negate: bool) {
    b.offset[AA] = if negate { -n } else { n };
    if axis != AA {
        b.min.swap(AA, axis);
        b.ps.iter_mut().for_each(|p| p.swap(AA, axis));
    }
    if negate {
        b.min[AA] = n - b.min[AA] - 2047;
        b.ps.iter_mut().for_each(|p| p[AA] = n - p[AA]);
    } else {
        b.min[AA] -= n;
        b.ps.iter_mut().for_each(|p| p[AA] -= n);
    }
}

fn combine(scanners: &mut [Scanner]) {
    let mut need = (1_u64 << scanners.len()) - 2;
    let mut todo = vec![0];
    while let Some(i) = todo.pop() {
        for j in bits(need) {
            if let Some((n, axis, negate)) = can_align::<0>(&scanners[i], &scanners[j]) {
                align::<0>(&mut scanners[j], n, axis, negate);
                if let Some((n, axis, negate)) = can_align::<1>(&scanners[i], &scanners[j]) {
                    align::<1>(&mut scanners[j], n, axis, negate);
                }
                if let Some((n, axis, negate)) = can_align::<2>(&scanners[i], &scanners[j]) {
                    align::<2>(&mut scanners[j], n, axis, negate);
                }
                need ^= 1 << j;
                todo.push(j);
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut scanners = parse_scanners(input);
    combine(&mut scanners);
    scanners
        .iter()
        .flat_map(|s| s.ps.iter().map(hash))
        .collect::<AHashSet<_>>()
        .len()
}

pub fn part2(input: &str) -> i32 {
    let mut scanners = parse_scanners(input);
    combine(&mut scanners);
    let mut result = 0;
    for a in &scanners {
        for b in &scanners {
            let dist = (a.offset[0] - b.offset[0]).abs()
                + (a.offset[1] - b.offset[1]).abs()
                + (a.offset[2] - b.offset[2]).abs();
            result = max(result, dist);
        }
    }
    result
}
