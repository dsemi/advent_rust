use crate::utils::UniqueIdx;
use ahash::AHashMap;
use genawaiter::stack::{let_gen_using, Co};
use regex::Regex;
use std::cmp::{max, min};

struct Graph {
    start: usize,
    flow_rates: Vec<usize>,
    working_valves: Vec<usize>,
    dist: Vec<Vec<usize>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap();
        let mut ui = UniqueIdx::new();
        let valves = input.lines().collect::<Vec<_>>();
        let mut dist = vec![vec![usize::MAX; valves.len()]; valves.len()];
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
        let working_valves = flow_rates
            .iter()
            .enumerate()
            .filter_map(|(i, &r)| (r > 0).then(|| i))
            .collect();
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

    async fn step(&self, time: usize, co: Co<'_, (usize, usize)>) {
        let mut stack = vec![(self.start, 0, 0, time)];
        while let Some((i, open_valves, pressure, time_left)) = stack.pop() {
            co.yield_((open_valves, pressure)).await;
            for j in self.working_valves.iter().copied() {
                let bit = 1 << j;
                if bit & open_valves == 0 && self.dist[i][j] < time_left - 1 {
                    let time_left = time_left - self.dist[i][j] - 1;
                    stack.push((
                        j,
                        open_valves | bit,
                        pressure + self.flow_rates[j] * time_left,
                        time_left,
                    ));
                }
            }
        }
    }
}

pub fn part1(input: &str) -> Option<usize> {
    let graph = Graph::new(input);
    let_gen_using!(gen, |co| graph.step(30, co));
    gen.into_iter().map(|(_, pressure)| pressure).max()
}

pub fn part2(input: &str) -> Option<usize> {
    let graph = Graph::new(input);
    let mut open_to_press = AHashMap::new();
    let_gen_using!(gen, |co| graph.step(26, co));
    for (open_valves, pressure) in gen {
        let e = open_to_press.entry(open_valves).or_insert(pressure);
        *e = max(*e, pressure);
    }
    let best_pressures = open_to_press.into_iter().collect::<Vec<_>>();
    best_pressures
        .iter()
        .enumerate()
        .flat_map(|(hi, (h_opens, h_pressure))| {
            best_pressures
                .iter()
                .skip(hi + 1)
                .filter_map(move |(e_opens, e_pressure)| {
                    (h_opens & e_opens == 0).then(|| h_pressure + e_pressure)
                })
        })
        .max()
}
