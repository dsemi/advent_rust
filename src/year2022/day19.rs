use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

const TMPL: &str = "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.";

fn gte(a: &[i32; 4], b: &[i32; 4]) -> bool {
    a.iter().zip(b).all(|(a, b)| a >= b)
}

fn add(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
    combine(a.iter().zip(b).map(|(a, b)| a + b))
}

fn sub(a: &[i32; 4], b: &[i32; 4]) -> [i32; 4] {
    combine(a.iter().zip(b).map(|(a, b)| a - b))
}

fn combine<I: Iterator<Item = i32>>(src: I) -> [i32; 4] {
    let mut result = [0, 0, 0, 0];
    for (r, v) in result.iter_mut().zip(src) {
        *r = v;
    }
    result
}

struct Blueprint {
    num: i32,
    costs: [[i32; 4]; 4],
    max_costs: [i32; 4],
}

fn blueprints(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(|line| {
        let (
            num,
            ore_bot_ore,
            clay_bot_ore,
            obs_bot_ore,
            obs_bot_clay,
            geode_bot_ore,
            geode_bot_obs,
        ) = scanf!(line, TMPL, i32, i32, i32, i32, i32, i32, i32).unwrap();
        let costs = [
            [0, geode_bot_obs, 0, geode_bot_ore],
            [0, 0, obs_bot_clay, obs_bot_ore],
            [0, 0, 0, clay_bot_ore],
            [0, 0, 0, ore_bot_ore],
        ];
        Blueprint {
            num,
            costs,
            max_costs: costs.iter().fold([0; 4], |mc, c| {
                combine(mc.iter().zip(c).map(|(a, b)| *max(a, b)))
            }),
        }
    })
}

impl Blueprint {
    fn dfs(&self, res: &mut i32, time: i32, amts: [i32; 4], bots: [i32; 4], mut bans: u8) {
        let geodes = amts[0];
        let geode_bots = bots[0];
        if time == 0 {
            *res = max(*res, geodes);
            return;
        }
        let mut upper_bd = geodes + time * geode_bots;
        let (mut obs, mut obs_rate, obs_cost) = (amts[1], bots[1], self.costs[0][1]);
        for t in (0..time).rev() {
            if obs >= obs_cost {
                obs += obs_rate - obs_cost;
                upper_bd += t;
            } else {
                obs += obs_rate;
                obs_rate += 1;
            }
        }
        if upper_bd <= *res {
            return;
        }
        for (i, costs) in self.costs.iter().enumerate() {
            if bans & (1 << i) == 0 && (i == 0 || bots[i] < self.max_costs[i]) && gte(&amts, costs)
            {
                let mut new_bots = [0; 4];
                new_bots[i] = 1;
                self.dfs(
                    res,
                    time - 1,
                    sub(&add(&amts, &bots), costs),
                    add(&bots, &new_bots),
                    0,
                );
                bans |= 1 << i;
            }
        }
        self.dfs(res, time - 1, add(&amts, &bots), bots, bans);
    }

    fn sim(&self, time: i32) -> i32 {
        let mut res = 0;
        self.dfs(&mut res, time, [0, 0, 0, 0], [0, 0, 0, 1], 0);
        res
    }
}

pub fn part1(input: &str) -> i32 {
    blueprints(input).map(|b| b.num * b.sim(24)).sum()
}

pub fn part2(input: &str) -> i32 {
    blueprints(input).take(3).map(|b| b.sim(32)).product()
}
