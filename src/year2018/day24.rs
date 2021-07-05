use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
struct Group {
    num: i32,
    name: String,
    units: i32,
    hit_pts: i32,
    dmg: i32,
    element: String,
    initiative: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

fn parse_armies(input: &str) -> HashMap<i32, Group> {
    input
        .split("\n\n")
        .flat_map(|a| {
            let mut gen = a.lines();
            let name = gen.next().unwrap();
            let re = Regex::new(r"(\d+) units each with (\d+) hit points(?: \((.+)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
            gen.map(move |line| {
                let cap = re.captures(line).unwrap();
                let units = cap[1].parse().unwrap();
                let hp = cap[2].parse().unwrap();
                let mods = cap.get(3);
                let dmg = cap[4].parse().unwrap();
                let element = cap[5].to_string();
                let initiative = cap[6].parse().unwrap();
                let mut group = Group {
                    num: 0,
                    name: name[..name.len() - 1].to_string(),
                    units: units,
                    hit_pts: hp,
                    dmg: dmg,
                    element: element,
                    initiative: initiative,
                    weaknesses: Vec::new(),
                    immunities: Vec::new(),
                };
                if let Some(mds) = mods {
                    for x in mds.as_str().split("; ") {
                        let (m, elems) = x.split_once(" to ").unwrap();
                        if m == "weak" {
                            group
                                .weaknesses
                                .extend(elems.split(", ").map(|x| x.to_string()));
                        } else {
                            group
                                .immunities
                                .extend(elems.split(", ").map(|x| x.to_string()));
                        }
                    }
                }
                group
            })
        })
        .zip(0..)
        .map(|(mut g, n)| {
            g.num = n;
            (n, g)
        })
        .collect()
}

impl Group {
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

fn select_target(
    groups: &HashMap<i32, Group>,
    attacked: &mut HashSet<i32>,
    grp: &Group,
) -> Option<i32> {
    let mut mx = (0, 0, 0, 0);
    for (i, g) in groups.iter() {
        if grp.name != g.name && !attacked.contains(i) {
            let mx2 = (*i, grp.calc_dmg(g), g.eff_pwr(), g.initiative);
            if (mx2.1, mx2.2, mx2.3) > (mx.1, mx.2, mx.3) {
                mx = mx2;
            }
        }
    }
    (mx.1 > 0).then(|| {
        attacked.insert(mx.0);
        mx.0
    })
}

fn target_selection(groups: &HashMap<i32, Group>) -> Vec<(i32, i32)> {
    let mut target_selectors = groups.values().collect::<Vec<_>>();
    target_selectors.sort_by_key(|g| (-g.eff_pwr(), -g.initiative));
    let mut s = HashSet::new();
    let mut res = target_selectors
        .into_iter()
        .filter_map(|g| select_target(groups, &mut s, g).map(|t| (g, t)))
        .collect::<Vec<_>>();
    res.sort_by_key(|(g, _)| -g.initiative);
    res.into_iter().map(|(g, t)| (g.num, t)).collect()
}

fn attack(groups: &mut HashMap<i32, Group>, atks: Vec<(i32, i32)>) -> bool {
    let mut result = false;
    for (k1, k2) in atks {
        if groups.contains_key(&k1) {
            let g1 = &groups[&k1];
            let g2 = &groups[&k2];
            let units_left = g2.units - g1.calc_dmg(&g2) / g2.hit_pts;
            if units_left != g2.units {
                result = true;
            }
            if units_left <= 0 {
                groups.remove(&k2);
            } else {
                let e = groups.get_mut(&k2).unwrap();
                e.units = units_left;
            }
        }
    }
    result
}

fn battle(groups: &mut HashMap<i32, Group>) -> bool {
    let mut changed = true;
    while changed {
        let mut gen = groups.values();
        let name = &gen.next().unwrap().name;
        if gen.all(|g| &g.name == name) {
            return true;
        }
        let atks = target_selection(groups);
        changed = attack(groups, atks);
    }
    false
}

pub fn part1(input: &str) -> i32 {
    let mut groups = parse_armies(input);
    battle(&mut groups);
    groups.values().map(|g| g.units).sum()
}

pub fn part2(input: &str) -> i32 {
    let gps = parse_armies(input);
    for n in 0.. {
        let mut groups = gps.clone();
        for g in groups.values_mut() {
            if g.name == "Immune System" {
                g.dmg += n;
            }
        }
        if battle(&mut groups) {
            let result = groups.values().filter_map(|g| (g.name == "Immune System").then(|| g.units)).sum();
            if result > 0 {
                return result;
            }
        }
    }
    panic!("No solution found")
}
