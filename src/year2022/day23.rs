use crate::utils::Coord;
use ahash::AHashSet;
use std::cmp::{max, min};

const DIRS: &[Coord<i16>] = &[
    Coord::new(-1, 1),
    Coord::new(0, 1),
    Coord::new(1, 1),
    Coord::new(-1, 0),
    Coord::new(1, 0),
    Coord::new(-1, -1),
    Coord::new(-0, -1),
    Coord::new(1, -1),
];

fn mv(elves: &AHashSet<Coord<i16>>, elf: &Coord<i16>, dir: usize) -> Coord<i16> {
    let adjs: Vec<bool> = DIRS.iter().map(|d| !elves.contains(&(elf + d))).collect();
    if !adjs.iter().all(|b| *b) {
        let poss = vec![
            (adjs[0] && adjs[1] && adjs[2], *elf + Coord::new(0, 1)),
            (adjs[5] && adjs[6] && adjs[7], *elf + Coord::new(0, -1)),
            (adjs[0] && adjs[3] && adjs[5], *elf + Coord::new(-1, 0)),
            (adjs[2] && adjs[4] && adjs[7], *elf + Coord::new(1, 0)),
        ];
        for i in 0..4 {
            let (avail, elf2) = poss[(dir + i) % 4];
            if avail {
                return elf2;
            }
        }
    }
    *elf
}

fn parse(input: &str) -> AHashSet<Coord<i16>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(c, v)| (v == '#').then(|| Coord::new(c as i16, -(r as i16))))
        })
        .collect()
}

fn step(elves: &mut AHashSet<Coord<i16>>, dir: usize) {
    let elves2 = elves.clone();
    elves.clear();
    for elf in elves2.iter() {
        let elf2 = mv(&elves2, elf, dir);
        if !elves.insert(elf2) {
            elves.remove(&elf2);
            elves.insert(*elf);
            elves.insert(elf2.scale(2) - *elf);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut elves = parse(input);
    for i in 0..10 {
        step(&mut elves, i % 4);
    }
    let (mut min_x, mut min_y) = (i16::MAX, i16::MAX);
    let (mut max_x, mut max_y) = (i16::MIN, i16::MIN);
    for elf in elves.iter() {
        min_x = min(min_x, elf.x);
        min_y = min(min_y, elf.y);
        max_x = max(max_x, elf.x + 1);
        max_y = max(max_y, elf.y + 1);
    }
    (max_x - min_x) as usize * (max_y - min_y) as usize - elves.len()
}

pub fn part2(input: &str) -> usize {
    let mut elves = parse(input);
    for i in 0.. {
        let prev = elves.clone();
        step(&mut elves, i % 4);
        if prev == elves {
            return i + 1;
        }
    }
    unreachable!()
}
