use crate::utils::parsers::*;
use derive_more::{Add, AddAssign, Constructor, Sub};
use std::cmp::{Ordering, Ordering::*, PartialOrd, max};
use std::mem::replace;

const ORE_BOT: Res = Res::new(1, 0, 0, 0);
const CLAY_BOT: Res = Res::new(0, 1, 0, 0);
const OBS_BOT: Res = Res::new(0, 0, 1, 0);
const GEODE_BOT: Res = Res::new(0, 0, 0, 1);

#[derive(Add, AddAssign, Clone, Constructor, Copy, PartialEq, Sub)]
struct Res {
    ore: u16,
    clay: u16,
    obs: u16,
    geode: u16,
}

impl PartialOrd for Res {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let a = self.ore.cmp(&rhs.ore);
        let b = self.clay.cmp(&rhs.clay);
        let c = self.obs.cmp(&rhs.obs);
        (a.min(b).min(c) != Less || a.max(b).max(c) != Greater).then_some(a.then(b).then(c))
    }
}

struct Blueprint {
    num: u16,
    ore_cost: Res,
    clay_cost: Res,
    obs_cost: Res,
    geode_cost: Res,
    max_ore: u16,
    max_clay: u16,
    max_obs: u16,
}

fn blueprints(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(|line| {
        let line = line.replace(|c: char| !c.is_ascii_digit(), " ");
        let ns: Vec<_> = spaced(u16).read(line.as_str());
        let [
            num,
            ore_bot_ore,
            clay_bot_ore,
            obs_bot_ore,
            obs_bot_clay,
            geode_bot_ore,
            geode_bot_obs,
        ] = ns.try_into().unwrap();
        Blueprint {
            num,
            ore_cost: Res::new(ore_bot_ore, 0, 0, 0),
            clay_cost: Res::new(clay_bot_ore, 0, 0, 0),
            obs_cost: Res::new(obs_bot_ore, obs_bot_clay, 0, 0),
            geode_cost: Res::new(geode_bot_ore, 0, geode_bot_obs, 0),
            max_ore: ore_bot_ore.max(clay_bot_ore).max(obs_bot_ore).max(geode_bot_ore),
            max_clay: obs_bot_clay,
            max_obs: geode_bot_obs,
        }
    })
}

impl Blueprint {
    fn dfs(&self, max_geode: &mut u16, time: u16, amts: Res, bots: Res) {
        *max_geode = max(*max_geode, amts.geode + time * bots.geode);
        if self.upper_bd(time, amts, bots) <= *max_geode {
            return;
        }

        if bots.obs > 0 && time > 1 {
            self.make_bot(max_geode, time, amts, bots, GEODE_BOT, self.geode_cost);
        }
        if bots.obs < self.max_obs && bots.clay > 0 && time > 3 {
            self.make_bot(max_geode, time, amts, bots, OBS_BOT, self.obs_cost);
        }
        if bots.ore < self.max_ore && time > 3 {
            self.make_bot(max_geode, time, amts, bots, ORE_BOT, self.ore_cost);
        }
        if bots.clay < self.max_clay && time > 5 {
            self.make_bot(max_geode, time, amts, bots, CLAY_BOT, self.clay_cost);
        }
    }

    fn upper_bd(&self, time: u16, mut amts: Res, mut bots: Res) -> u16 {
        for _ in 0..time {
            amts.ore = self.max_ore;
            if self.geode_cost <= amts {
                amts += bots - self.geode_cost;
                bots += GEODE_BOT;
            } else if self.obs_cost <= amts {
                amts += bots - self.obs_cost;
                bots += OBS_BOT;
            } else {
                amts += bots;
            }
            bots += CLAY_BOT;
        }
        amts.geode
    }

    fn make_bot(&self, max: &mut u16, time: u16, amts: Res, bots: Res, new_bot: Res, cost: Res) {
        if let Some((t, amts)) = (1..time)
            .scan((1, amts), |a, _| Some(replace(a, (a.0 + 1, a.1 + bots))))
            .find(|&(_, amts)| cost <= amts)
        {
            self.dfs(max, time - t, amts + bots - cost, bots + new_bot);
        }
    }

    fn sim(&self, time: u16) -> u16 {
        let mut res = 0;
        self.dfs(&mut res, time, Res::new(0, 0, 0, 0), ORE_BOT);
        res
    }
}

pub fn part1(input: &str) -> u16 {
    blueprints(input).map(|b| b.num * b.sim(24)).sum()
}

pub fn part2(input: &str) -> u16 {
    blueprints(input).take(3).map(|b| b.sim(32)).product()
}
