use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashMap;
use itertools::Itertools;
use std::cmp::{max, Reverse};

fn valve(i: &str) -> IResult<&str, Valve<'_>> {
    let (i, name) = preceded(tag("Valve "), alpha1)(i)?;
    let (i, flow) = delimited(tag(" has flow rate="), u8, tag("; "))(i)?;
    let (i, tunnel) = list(alpha1)(i.splitn(5, ' ').last().unwrap())?;
    Ok((i, Valve { name, flow, tunnel }))
}

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow: u8,
    tunnel: Vec<&'a str>,
}

struct Graph {
    flow: Vec<u8>,
    valves: u16,
    dist: Vec<Vec<u8>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut valves: Vec<Valve<'_>> = input.lines().map(|line| valve(line).unwrap().1).collect();
        valves.sort_unstable_by(|a, b| b.flow.cmp(&a.flow).then(a.name.cmp(b.name)));
        let ui: AHashMap<&str, usize> = valves.iter().map(|v| v.name).zip(0..).collect();
        let mut dist = vec![vec![u8::MAX; valves.len()]; valves.len()];
        let mut flow = vec![0; valves.len()];
        for valve in valves {
            let i = ui[valve.name];
            dist[i][i] = 0;
            flow[i] = valve.flow;
            for tunnel in valve.tunnel {
                dist[i][ui[tunnel]] = 1;
            }
        }
        floyd_warshall(&mut dist);
        let size = ui["AA"] + 1;
        flow.truncate(size);
        dist.truncate(size);
        dist.iter_mut().for_each(|row| row.truncate(size));
        let valves = (1 << ui["AA"]) - 1;
        Graph { flow, valves, dist }
    }

    fn dfs(&self, res: &mut [u16], i: usize, closed: u16, open: u16, pressure: u16, time_left: u8) {
        let upper_bd = self.upper_bound(closed, pressure, time_left);
        let e = res.get_mut(open as usize % res.len()).unwrap();
        if upper_bd <= *e {
            return;
        }
        *e = max(*e, pressure);
        for j in bits(closed) {
            let bit = 1 << j;
            if self.dist[i][j] < time_left - 1 {
                let time_left = time_left - self.dist[i][j] - 1;
                self.dfs(
                    res,
                    j,
                    closed & !bit,
                    open | bit,
                    pressure + self.flow[j] as u16 * time_left as u16,
                    time_left,
                );
            }
        }
    }

    fn sim(&self, time: u8, bins: usize) -> Vec<u16> {
        let mut res = vec![0; bins];
        let start = self.valves.count_ones() as usize;
        self.dfs(&mut res, start, self.valves, 0, 0, time);
        res
    }

    fn upper_bound(&self, closed: u16, pressure: u16, time_left: u8) -> u16 {
        (2..=time_left - 2)
            .rev()
            .step_by(2)
            .zip(bits(closed).map(|j| self.flow[j]))
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
        .filter_map(|(i, best)| (best > 0).then_some((i as u16, best)))
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
                .find_map(|(e_opens, p)| (e_opens & h_opens == 0).then_some(p))
                .unwrap_or(best)
        })
}
