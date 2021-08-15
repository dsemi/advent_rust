use ahash::AHashMap;
use ahash::AHashSet;
use itertools::Itertools;
use regex::Regex;

use crate::utils::*;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pair {
    chip: i32,
    gen: i32,
}

#[derive(Eq, Clone, Hash, PartialEq)]
struct Floors {
    elev: i32,
    flrs: Vec<Pair>,
}

impl Floors {
    fn is_valid(&self) -> bool {
        for p in self.flrs.iter() {
            if p.chip != p.gen && self.flrs.iter().any(|x| x.gen == p.chip) {
                return false;
            }
        }
        true
    }

    fn is_done(&self) -> bool {
        self.flrs.iter().all(|x| x.chip == 4 && x.gen == 4)
    }
}

fn parse_floors(input: &str) -> Floors {
    let mut tbl = AHashMap::new();
    let re = Regex::new(r"\S+ (microchip|generator)").unwrap();
    for (i, line) in input.lines().enumerate() {
        for item in re.find_iter(line) {
            match item.as_str().split_once(' ').unwrap() {
                (radiation, "generator") => {
                    let e = tbl.entry(radiation).or_insert(Pair { chip: 0, gen: 0 });
                    e.gen = i as i32 + 1;
                }
                (compat, "microchip") => {
                    let e = tbl
                        .entry(compat.split_once('-').unwrap().0)
                        .or_insert(Pair { chip: 0, gen: 0 });
                    e.chip = i as i32 + 1;
                }
                _ => panic!("Invalid input: {}", line),
            }
        }
    }
    Floors {
        elev: 1,
        flrs: tbl.values().copied().sorted().collect(),
    }
}

fn all_moves(floors: &Floors, e: i32) -> Vec<Floors> {
    let mut result = Vec::new();
    for i in 0..floors.flrs.len() {
        if floors.flrs[i].chip == floors.elev {
            let mut floors2 = floors.clone();
            floors2.flrs[i].chip = e;
            result.push(floors2);
        }
        if floors.flrs[i].gen == floors.elev {
            let mut floors2 = floors.clone();
            floors2.flrs[i].gen = e;
            result.push(floors2);
        }
    }
    result
}

fn neighbors(floors: &Floors) -> Vec<Floors> {
    let mut result = Vec::new();
    let mut neighbs = AHashSet::new();
    for e in &[floors.elev + 1, floors.elev - 1] {
        if *e > 0 && *e <= 4 {
            for mut floors2 in all_moves(floors, *e) {
                if floors2.is_valid() {
                    floors2.flrs.sort();
                    let neighb = Floors {
                        elev: *e,
                        flrs: floors2.flrs.clone(),
                    };
                    if !neighbs.contains(&neighb) {
                        neighbs.insert(neighb.clone());
                        result.push(neighb);
                    }
                }
                for mut floors3 in all_moves(&floors2, *e) {
                    if floors3.is_valid() {
                        floors3.flrs.sort();
                        let neighb = Floors {
                            elev: *e,
                            flrs: floors3.flrs,
                        };
                        if !neighbs.contains(&neighb) {
                            neighbs.insert(neighb.clone());
                            result.push(neighb);
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn part1(input: &str) -> Option<usize> {
    bfs(parse_floors(input), neighbors)
        .filter_map(|(d, st)| st.is_done().then(|| d))
        .next()
}

pub fn part2(input: &str) -> Option<usize> {
    let mut floors = parse_floors(input);
    floors.flrs.insert(0, Pair { chip: 1, gen: 1 });
    floors.flrs.insert(0, Pair { chip: 1, gen: 1 });
    bfs(floors, neighbors)
        .filter_map(|(d, st)| st.is_done().then(|| d))
        .next()
}
