use crate::utils::parsers::*;
use itertools::iterate;

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

fn attributes<'a>(i: &mut &'a str) -> Result<(Vec<&'a str>, Vec<&'a str>)> {
    '('.parse_next(i)?;
    let [weak, immune] = repeat(
        0..=2,
        terminated(
            alt((
                preceded("weak to ", list(alpha1)).map(|w| (0, w)),
                preceded("immune to ", list(alpha1)).map(|i| (1, i)),
            )),
            opt("; "),
        ),
    )
    .fold(Default::default, |mut acc: [Vec<&str>; 2], (i, xs)| {
        acc[i].extend(xs);
        acc
    })
    .parse_next(i)?;
    ") ".parse_next(i)?;
    Ok((weak, immune))
}

fn units<'a>(name: &'a str) -> impl Parser<&'a str, Option<Group<'a>>, ContextError> {
    move |i: &mut &'a str| {
        let (units, _, hp, _, attr, _, dmg, _, elem, _, init) = (
            i32,
            " units each with ",
            i32,
            " hit points ",
            opt(attributes),
            "with an attack that does ",
            i32,
            ' ',
            alpha1,
            " damage at initiative ",
            i32,
        )
            .parse_next(i)?;
        let (weaknesses, immunities) = attr.unwrap_or_default();
        let group = Group {
            num: 0,
            name,
            units,
            hit_pts: hp,
            dmg,
            element: elem,
            initiative: init,
            weaknesses,
            immunities,
        };
        Ok(Some(group))
    }
}

fn army<'a>(i: &mut &'a str) -> Result<Vec<Option<Group<'a>>>> {
    let name = terminated(take_till(1.., |c| c == ':'), ":\n").parse_next(i)?;
    lines(units(name)).parse_next(i)
}

fn armies<'a>(i: &mut &'a str) -> Result<Vec<Option<Group<'a>>>> {
    let (a, b) = sep2(army, "\n\n").parse_next(i)?;
    let mut result = [a, b].concat();
    result.iter_mut().filter_map(|g| g.as_mut()).enumerate().for_each(|(i, g)| g.num = i);
    Ok(result)
}

fn select_target(groups: &[Option<Group>], attacked: &mut u32, grp: &Group) -> Option<usize> {
    let mut mx = (0, 0, 0, 0);
    for g in groups.iter().flatten() {
        if grp.name != g.name && *attacked & (1 << g.num) == 0 {
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
        let mut grps = groups.iter().flatten();
        let name = grps.next().unwrap().name;
        if grps.all(|g| g.name == name) {
            return true;
        }
        let atks = target_selection(groups);
        changed = attack(groups, atks);
    }
    false
}

pub fn part1(input: &str) -> i32 {
    let mut groups = armies.read(input);
    battle(&mut groups);
    groups.iter().flatten().map(|g| g.units).sum()
}

fn immune_left(gps: &[Option<Group>], n: i32) -> i32 {
    let mut groups = gps.to_owned();
    groups.iter_mut().flatten().filter(|g| g.name == "Immune System").for_each(|g| g.dmg += n);
    if battle(&mut groups) {
        groups.iter().flatten().filter_map(|g| (g.name == "Immune System").then_some(g.units)).sum()
    } else {
        0
    }
}

pub fn part2(input: &str) -> i32 {
    let gps = armies.read(input);
    let hi = iterate(1, |x| x * 2).map(|n| immune_left(&gps, n)).find(|&left| left > 0).unwrap();
    let boosts = (0..hi).collect::<Vec<_>>();
    let boost = boosts.partition_point(|&n| immune_left(&gps, n) <= 0) as i32;
    immune_left(&gps, boost)
}
