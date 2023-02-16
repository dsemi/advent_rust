use itertools::iterate;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, i32};
use nom::combinator::opt;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;
use std::cell::Cell;

#[derive(Clone)]
struct Group<'a> {
    num: usize,
    name: &'a str,
    units: i32,
    hit_pts: i32,
    dmg: i32,
    element: &'a str,
    initiative: i32,
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
}

impl Group<'_> {
    fn eff_pwr(&self) -> i32 {
        self.units * self.dmg
    }

    fn calc_dmg(&self, b: &Self) -> i32 {
        if b.weaknesses.contains(&self.element) {
            2 * self.eff_pwr()
        } else if b.immunities.contains(&self.element) {
            0
        } else {
            self.eff_pwr()
        }
    }
}

fn units<'a>(idx: &Cell<usize>, name: &'a str, i: &'a str) -> IResult<&'a str, Option<Group<'a>>> {
    let (i, units) = i32(i)?;
    let (i, hit_pts) = delimited(tag(" units each with "), i32, tag(" hit points "))(i)?;
    let (i, attributes) = opt(delimited(
        tag("("),
        separated_list0(
            tag("; "),
            separated_pair(
                alt((tag("weak"), tag("immune"))),
                tag(" to "),
                separated_list1(tag(", "), alpha1),
            ),
        ),
        tag(") "),
    ))(i)?;
    let (weaknesses, immunities) = attributes
        .unwrap_or_default()
        .into_iter()
        .partition::<Vec<(&str, Vec<&str>)>, _>(|x| x.0 == "weak");
    let (i, dmg) = delimited(tag("with an attack that does "), i32, tag(" "))(i)?;
    let (i, element) = alpha1(i)?;
    let (i, initiative) = preceded(tag(" damage at initiative "), i32)(i)?;
    let group = Group {
        num: idx.replace(idx.get() + 1),
        name,
        units,
        hit_pts,
        dmg,
        element,
        initiative,
        weaknesses: weaknesses.into_iter().flat_map(|x| x.1).collect(),
        immunities: immunities.into_iter().flat_map(|x| x.1).collect(),
    };
    Ok((i, Some(group)))
}

fn army<'a>(idx: &Cell<usize>, i: &'a str) -> IResult<&'a str, Vec<Option<Group<'a>>>> {
    let (i, name) = terminated(take_till(|c: char| c == ':'), tag(":\n"))(i)?;
    separated_list1(tag("\n"), |i| units(idx, name, i))(i)
}

fn armies(i: &str) -> IResult<&str, Vec<Option<Group>>> {
    let idx = Cell::new(0);
    let (i, (a, b)) = separated_pair(|i| army(&idx, i), tag("\n\n"), |i| army(&idx, i))(i)?;
    Ok((i, [a, b].concat()))
}

fn select_target(groups: &[Option<Group>], attacked: &mut u32, grp: &Group) -> Option<usize> {
    let mut mx = (0, 0, 0, 0);
    for g in groups.iter().flatten() {
        if grp.name != g.name && *attacked & 1 << g.num == 0 {
            let mx2 = (g.num, grp.calc_dmg(g), g.eff_pwr(), g.initiative);
            if (mx2.1, mx2.2, mx2.3) > (mx.1, mx.2, mx.3) {
                mx = mx2;
            }
        }
    }
    (mx.1 > 0).then(|| {
        *attacked |= 1 << mx.0;
        mx.0
    })
}

fn target_selection(groups: &[Option<Group>]) -> Vec<(usize, usize)> {
    let mut target_selectors = groups.iter().flatten().collect::<Vec<_>>();
    target_selectors.sort_by_key(|g| (-g.eff_pwr(), -g.initiative));
    let mut s = 0;
    let mut res = target_selectors
        .into_iter()
        .filter_map(|g| select_target(groups, &mut s, g).map(|t| (g, t)))
        .collect::<Vec<_>>();
    res.sort_by_key(|(g, _)| -g.initiative);
    res.into_iter().map(|(g, t)| (g.num, t)).collect()
}

fn attack(groups: &mut [Option<Group>], atks: Vec<(usize, usize)>) -> bool {
    let mut result = false;
    for (k1, k2) in atks {
        if let Some(g1) = groups[k1].as_ref() {
            let g2 = groups[k2].as_ref().unwrap();
            let units_left = g2.units - g1.calc_dmg(g2) / g2.hit_pts;
            if units_left != g2.units {
                result = true;
            }
            if units_left <= 0 {
                groups[k2] = None;
            } else {
                groups[k2].as_mut().unwrap().units = units_left;
            }
        }
    }
    result
}

fn battle(groups: &mut [Option<Group>]) -> bool {
    let mut changed = true;
    while changed {
        let mut gen = groups.iter().flatten();
        let name = gen.next().unwrap().name;
        if gen.all(|g| g.name == name) {
            return true;
        }
        let atks = target_selection(groups);
        changed = attack(groups, atks);
    }
    false
}

pub fn part1(input: &str) -> i32 {
    let mut groups = armies(input).unwrap().1;
    battle(&mut groups);
    groups.iter().flatten().map(|g| g.units).sum()
}

fn immune_left(gps: &[Option<Group>], n: i32) -> i32 {
    let mut groups = gps.to_owned();
    groups
        .iter_mut()
        .flatten()
        .filter(|g| g.name == "Immune System")
        .for_each(|g| g.dmg += n);
    if battle(&mut groups) {
        groups
            .iter()
            .flatten()
            .filter_map(|g| (g.name == "Immune System").then(|| g.units))
            .sum()
    } else {
        0
    }
}

pub fn part2(input: &str) -> i32 {
    let gps = armies(input).unwrap().1;
    let hi = iterate(1, |x| x * 2)
        .map(|n| immune_left(&gps, n))
        .find(|&left| left > 0)
        .unwrap();
    let boosts = (0..hi).collect::<Vec<_>>();
    let boost = boosts.partition_point(|&n| immune_left(&gps, n) <= 0) as i32;
    immune_left(&gps, boost)
}
