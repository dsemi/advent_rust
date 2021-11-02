use regex::Regex;

#[derive(Clone)]
struct Group {
    num: usize,
    name: String,
    units: i32,
    hit_pts: i32,
    dmg: i32,
    element: String,
    initiative: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

fn parse_armies(input: &str) -> Vec<Option<Group>> {
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
                    units,
                    hit_pts: hp,
                    dmg,
                    element,
                    initiative,
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
        .enumerate()
        .map(|(n, mut g)| {
            g.num = n;
            Some(g)
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

fn attack(groups: &mut Vec<Option<Group>>, atks: Vec<(usize, usize)>) -> bool {
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

fn battle(groups: &mut Vec<Option<Group>>) -> bool {
    let mut changed = true;
    while changed {
        let mut gen = groups.iter().flatten();
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
    groups.iter().flatten().map(|g| g.units).sum()
}

pub fn part2(input: &str) -> i32 {
    let gps = parse_armies(input);
    for n in 0.. {
        let mut groups = gps.clone();
        for g in groups.iter_mut().flatten() {
            if g.name == "Immune System" {
                g.dmg += n;
            }
        }
        if battle(&mut groups) {
            let result = groups
                .iter()
                .flatten()
                .filter_map(|g| (g.name == "Immune System").then(|| g.units))
                .sum();
            if result > 0 {
                return result;
            }
        }
    }
    panic!("No solution found")
}
