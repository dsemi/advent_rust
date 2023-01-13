use crate::utils::UniqueIdx;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};

struct Graph {
    start: usize,
    flow_rates: Vec<u8>,
    working_valves: Vec<usize>,
    dist: Vec<Vec<u8>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap();
        let mut ui = UniqueIdx::new();
        let valves = input.lines().collect::<Vec<_>>();
        let mut dist = vec![vec![u8::MAX; valves.len()]; valves.len()];
        let mut flow_rates = vec![0; valves.len()];
        for line in valves {
            let cap = re.captures(line).unwrap();
            let i = ui.idx(cap[1].to_owned());
            dist[i][i] = 0;
            flow_rates[i] = cap[2].parse().unwrap();
            for tunnel in cap[3].split(", ") {
                let j = ui.idx(tunnel.to_owned());
                dist[i][j] = 1;
            }
        }
        let mut working_valves = flow_rates
            .iter()
            .enumerate()
            .filter_map(|(i, &r)| (r > 0).then(|| i))
            .collect::<Vec<_>>();
        working_valves.sort_unstable_by_key(|&i| Reverse(flow_rates[i]));
        for k in 0..dist.len() {
            for i in 0..dist.len() {
                for j in 0..dist.len() {
                    dist[i][j] = min(dist[i][j], dist[i][k].saturating_add(dist[k][j]));
                }
            }
        }
        Graph {
            start: ui.idx("AA".to_owned()),
            flow_rates,
            working_valves,
            dist,
        }
    }

    fn dfs(&self, res: &mut [u16], i: usize, open_valves: u16, pressure: u16, time_left: u8) {
        let upper_bd = self.upper_bound(open_valves, pressure, time_left);
        let e = res.get_mut(open_valves as usize % res.len()).unwrap();
        if upper_bd <= *e {
            return;
        }
        *e = max(*e, pressure);
        for (bit, j) in self.working_valves.iter().copied().enumerate() {
            let bit = 1 << bit;
            if bit & open_valves == 0 && self.dist[i][j] < time_left - 1 {
                let time_left = time_left - self.dist[i][j] - 1;
                self.dfs(
                    res,
                    j,
                    open_valves | bit,
                    pressure + self.flow_rates[j] as u16 * time_left as u16,
                    time_left,
                );
            }
        }
    }

    fn sim(&self, time: u8, bins: usize) -> Vec<u16> {
        let mut res = vec![0; bins];
        self.dfs(&mut res, self.start, 0, 0, time);
        res
    }

    fn upper_bound(&self, open_valves: u16, pressure: u16, time_left: u8) -> u16 {
        (0..=time_left)
            .rev()
            .step_by(2)
            .skip(1)
            .zip(
                self.working_valves
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &j)| {
                        (open_valves & (1 << i) == 0).then(|| self.flow_rates[j])
                    }),
            )
            .map(|(min, flow)| min as u16 * flow as u16)
            .chain(std::iter::once(pressure))
            .sum()
    }
}

pub fn part1(input: &str) -> Option<u16> {
    let graph = Graph::new(input);
    graph.sim(30, 1).into_iter().max()
}

pub fn part2(input: &str) -> u16 {
    let graph = Graph::new(input);
    let best_pressures = graph
        .sim(26, u16::MAX as usize)
        .into_iter()
        .enumerate()
        .filter_map(|(i, best)| (best > 0).then(|| (i as u16, best)))
        .sorted_unstable_by_key(|p| Reverse(p.1))
        .collect::<Vec<_>>();
    best_pressures
        .iter()
        .enumerate()
        .fold(0, |best, (i, (h_opens, h_pressure))| {
            best_pressures
                .iter()
                .skip(i + 1)
                .map(|(o, e_pressure)| (o, h_pressure + e_pressure))
                .take_while(|&(_, p)| p > best)
                .find_map(|(e_opens, p)| (e_opens & h_opens == 0).then(|| p))
                .unwrap_or(best)
        })
}
