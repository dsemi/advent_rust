use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashMap;
use ahash::AHashSet;
use itertools::Itertools;
use std::ops::BitOrAssign;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pair {
    chip: i32,
    gen: i32,
}

impl BitOrAssign for Pair {
    fn bitor_assign(&mut self, rhs: Pair) {
        self.chip |= rhs.chip;
        self.gen |= rhs.gen;
    }
}

#[derive(Eq, Clone, Hash, PartialEq)]
struct Floors {
    elev: i32,
    flrs: Vec<Pair>,
}

impl Floors {
    fn is_valid(&self) -> bool {
        self.flrs
            .iter()
            .all(|p| p.chip == p.gen || self.flrs.iter().all(|x| x.gen != p.chip))
    }

    fn is_done(&self) -> bool {
        self.flrs.iter().all(|x| x.chip == 4 && x.gen == 4)
    }
}

fn parse(flr: i32) -> impl FnMut(&str) -> IResult<&str, (&str, Pair)> {
    move |i| {
        let (i, elem) = delimited(
            pair(opt(tag("and ")), tag("a ")),
            alpha1,
            pair(opt(tag("-compatible")), space1),
        )(i)?;
        let (i, pair) = alt((
            value(Pair { chip: flr, gen: 0 }, tag("microchip")),
            value(Pair { chip: 0, gen: flr }, tag("generator")),
        ))(i)?;
        Ok((i, (elem, pair)))
    }
}

fn parse_floors(input: &str) -> Floors {
    let mut tbl = AHashMap::new();
    for (i, line) in input.lines().enumerate() {
        if let Some(idx) = line.find("a ") {
            list(parse(i as i32 + 1))
                .read(&line[idx..])
                .into_iter()
                .for_each(|(k, pair)| *tbl.entry(k).or_default() |= pair);
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
    for e in [floors.elev + 1, floors.elev - 1] {
        if e > 0 && e <= 4 {
            for mut floors2 in all_moves(floors, e) {
                if floors2.is_valid() {
                    floors2.flrs.sort();
                    let neighb = Floors {
                        elev: e,
                        flrs: floors2.flrs.clone(),
                    };
                    if !neighbs.contains(&neighb) {
                        neighbs.insert(neighb.clone());
                        result.push(neighb);
                    }
                }
                for mut floors3 in all_moves(&floors2, e) {
                    if floors3.is_valid() {
                        floors3.flrs.sort();
                        let neighb = Floors {
                            elev: e,
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
    bfs(parse_floors(input), neighbors).find_map(|(d, st)| st.is_done().then_some(d))
}

pub fn part2(input: &str) -> Option<usize> {
    let mut floors = parse_floors(input);
    floors.flrs.insert(0, Pair { chip: 1, gen: 1 });
    floors.flrs.insert(0, Pair { chip: 1, gen: 1 });
    bfs(floors, neighbors).find_map(|(d, st)| st.is_done().then_some(d))
}
