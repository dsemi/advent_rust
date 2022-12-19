use safe_arch::*;
use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

const TMPL: &str = "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.";

#[allow(non_camel_case_types)]
union m128i_32 {
    arr: [i32; 4],
    v: m128i,
}

impl m128i_32 {
    unsafe fn gte(&self, o: &m128i_32) -> bool {
        move_mask_i8_m128i(cmp_lt_mask_i32_m128i(self.v, o.v)) == 0
    }

    unsafe fn add(&self, o: &m128i_32) -> m128i_32 {
        m128i_32 {
            v: add_i32_m128i(self.v, o.v),
        }
    }

    unsafe fn sub(&self, o: &m128i_32) -> m128i_32 {
        m128i_32 {
            v: sub_i32_m128i(self.v, o.v),
        }
    }
}

struct Blueprint {
    num: i32,
    costs: [m128i_32; 4],
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
            m128i_32 {
                arr: [0, geode_bot_obs, 0, geode_bot_ore],
            },
            m128i_32 {
                arr: [0, 0, obs_bot_clay, obs_bot_ore],
            },
            m128i_32 {
                arr: [0, 0, 0, clay_bot_ore],
            },
            m128i_32 {
                arr: [0, 0, 0, ore_bot_ore],
            },
        ];
        let mut mc = m128i::from([0i32, 0, 0, 0]);
        unsafe {
            for c in costs.iter() {
                mc = max_i32_m128i(mc, c.v);
            }
        }
        Blueprint {
            num,
            costs,
            max_costs: mc.into(),
        }
    })
}

impl Blueprint {
    unsafe fn dfs(&self, res: &mut i32, time: i32, amts: m128i_32, bots: m128i_32, mut bans: u8) {
        let geodes = amts.arr[0];
        let geode_bots = bots.arr[0];
        if time == 0 {
            *res = max(*res, geodes);
            return;
        }
        let upper_bd = geodes + time * geode_bots + time * (time + 1) / 2;
        if upper_bd <= *res {
            return;
        }
        for (i, costs) in self.costs.iter().enumerate() {
            if bans & (1 << i) == 0
                && (i == 0 || bots.arr[i] < self.max_costs[i])
                && amts.gte(costs)
            {
                let mut chans = m128i_32 { arr: [0, 0, 0, 0] };
                chans.arr[i] = 1;
                self.dfs(
                    res,
                    time - 1,
                    amts.add(&bots).sub(costs),
                    bots.add(&chans),
                    0,
                );
                bans |= 1 << i;
            }
        }
        self.dfs(res, time - 1, amts.add(&bots), bots, bans);
    }

    fn sim(&self, time: i32) -> i32 {
        let mut res = 0;
        unsafe {
            self.dfs(
                &mut res,
                time,
                m128i_32 { arr: [0, 0, 0, 0] },
                m128i_32 { arr: [0, 0, 0, 1] },
                0,
            );
        }
        res
    }
}

pub fn part1(input: &str) -> i32 {
    blueprints(input).map(|b| b.num * b.sim(24)).sum()
}

pub fn part2(input: &str) -> i32 {
    blueprints(input).take(3).map(|b| b.sim(32)).product()
}
