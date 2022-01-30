use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;
use std::cell::RefCell;
use std::cmp::{max, min};

#[derive(Debug)]
struct Pt {
    c: RefCell<[i32; 3]>,
}

impl Pt {
    fn mini(&self, other: &Pt) -> Pt {
        let a = *self.c.borrow();
        let b = *other.c.borrow();
        Pt {
            c: RefCell::new([min(a[0], b[0]), min(a[1], b[1]), min(a[2], b[2])]),
        }
    }

    fn hash(&self) -> u64 {
        let mut hash = 0;
        for n in *self.c.borrow() {
            hash = (hash << 21) ^ n as u64;
        }
        hash
    }
}

#[derive(Debug)]
struct Scanner {
    ps: Vec<Pt>,
    offset: Pt,
    min: Pt,
}

impl Scanner {
    fn add(&mut self, p: Pt) {
        self.min = self.min.mini(&p);
        self.ps.push(p);
    }
}

fn parse_scanners(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|sc| {
            let mut scanner = Scanner {
                ps: vec![],
                offset: Pt {
                    c: RefCell::new([0; 3]),
                },
                min: Pt {
                    c: RefCell::new([0; 3]),
                },
            };
            for line in sc.lines().skip(1) {
                let (x, y, z) = scanf!(line, "{},{},{}", i32, i32, i32).unwrap();
                scanner.add(Pt {
                    c: RefCell::new([x, y, z]),
                });
            }
            scanner
        })
        .collect()
}

fn align(a: &Scanner, b: &Scanner, aa: usize) -> bool {
    let mut collision = [0_u8; 4096 * 6];
    for pa in &a.ps {
        for pb in &b.ps {
            let mut base = 0;
            let vals = [
                2048 + (pb.c.borrow()[0] - b.min.c.borrow()[0])
                    - (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
                (pb.c.borrow()[0] - b.min.c.borrow()[0])
                    + (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
                2048 + (pb.c.borrow()[1] - b.min.c.borrow()[1])
                    - (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
                (pb.c.borrow()[1] - b.min.c.borrow()[1])
                    + (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
                2048 + (pb.c.borrow()[2] - b.min.c.borrow()[2])
                    - (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
                (pb.c.borrow()[2] - b.min.c.borrow()[2])
                    + (pa.c.borrow()[aa] - a.min.c.borrow()[aa]),
            ];
            for mut n in vals {
                let idx = base + n as usize;
                collision[idx] += 1;
                if collision[idx] == 12 {
                    let ori = idx / 4096;
                    let axis = ori / 2;
                    let negate = ori % 2 == 1;
                    n += b.min.c.borrow()[axis];
                    if negate {
                        n += a.min.c.borrow()[aa]
                    } else {
                        n -= a.min.c.borrow()[aa] + 2048;
                    }
                    b.offset.c.borrow_mut()[aa] = if negate { -n } else { n };
                    if axis != aa {
                        b.min.c.borrow_mut().swap(aa, axis);
                        for p in &b.ps {
                            p.c.borrow_mut().swap(aa, axis);
                        }
                    }
                    if negate {
                        let e = &mut b.min.c.borrow_mut()[aa];
                        *e = n - *e - 2047;
                        for p in &b.ps {
                            let e = &mut p.c.borrow_mut()[aa];
                            *e = n - *e;
                        }
                    } else {
                        let e = &mut b.min.c.borrow_mut()[aa];
                        *e -= n;
                        for p in &b.ps {
                            let e = &mut p.c.borrow_mut()[aa];
                            *e -= n;
                        }
                    }
                    return true;
                }
                base += 4096
            }
        }
    }
    false
}

struct Bits<T> {
    n: T,
}

impl Iterator for Bits<u64> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }
        let b = self.n.trailing_zeros();
        self.n &= self.n - 1;
        Some(b as usize)
    }
}

fn combine(input: &str) -> (AHashSet<u64>, Vec<Scanner>) {
    let scanners = parse_scanners(input);
    let mut set = AHashSet::new();
    let mut need = (1_u64 << scanners.len()) - 2;
    let mut todo = vec![0];
    while let Some(i) = todo.pop() {
        for j in (Bits { n: need }) {
            if align(&scanners[i], &scanners[j], 0) {
                align(&scanners[i], &scanners[j], 1);
                align(&scanners[i], &scanners[j], 2);
                need ^= 1 << j;
                todo.push(j);
            }
        }
    }
    for s in &scanners {
        for p in &s.ps {
            set.insert(p.hash());
        }
    }
    (set, scanners)
}

pub fn part1(input: &str) -> usize {
    combine(input).0.len()
}

pub fn part2(input: &str) -> i32 {
    let scanners = combine(input).1;
    let mut result = 0;
    for a in &scanners {
        for b in &scanners {
            let dist = (a.offset.c.borrow()[0] - b.offset.c.borrow()[0]).abs()
                + (a.offset.c.borrow()[1] - b.offset.c.borrow()[1]).abs()
                + (a.offset.c.borrow()[2] - b.offset.c.borrow()[2]).abs();
            result = max(result, dist);
        }
    }
    result
}
