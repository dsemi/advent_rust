use crate::utils::*;
use ahash::AHashMap;
use num::integer::lcm;
use std::collections::VecDeque;
use ModuleType::*;
use Pulse::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(u64, u32),
}

impl ModuleType {
    fn signal(&mut self, pulse: Pulse, src: usize) -> Option<Pulse> {
        match self {
            Broadcast => Some(pulse),
            FlipFlop(on) if pulse == Low => {
                *on = !*on;
                Some(if *on { High } else { Low })
            }
            Conjunction(ins, len) => {
                match pulse {
                    Low => *ins &= !(1 << src),
                    High => *ins |= 1 << src,
                }
                Some(if ins.count_ones() == *len { Low } else { High })
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Module {
    t: ModuleType,
    outs: Vec<usize>,
}

fn parse(input: &str) -> (usize, Vec<Module>) {
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .collect();
    let mut ui: UniqueIdx<_> = lines
        .iter()
        .map(|(k, _)| k.trim_start_matches(['%', '&']))
        .collect();
    let mut modules: Vec<_> = lines
        .into_iter()
        .map(|(k, v)| {
            let t = match k.chars().next().unwrap() {
                'b' => Broadcast,
                '%' => FlipFlop(false),
                '&' => Conjunction(0, 0),
                _ => unreachable!(),
            };
            let outs = v.split(", ").map(|o| ui.idx(o)).collect();
            Module { t, outs }
        })
        .collect();
    for i in 0..modules.len() {
        let outs = modules[i].outs.clone();
        for out in outs {
            if let Some(Conjunction(_, cnt)) = modules.get_mut(out).map(|x| &mut x.t) {
                *cnt += 1;
            }
        }
    }
    (ui.idx("broadcaster"), modules)
}

fn push_button(modules: &mut [Module], start: usize, mut f: impl FnMut(Pulse, usize, usize)) {
    let mut q = VecDeque::from([(Low, 0, start)]);
    while let Some((pulse, in_idx, idx)) = q.pop_front() {
        f(pulse, in_idx, idx);
        if let Some(m) = modules.get_mut(idx) {
            if let Some(pulse) = m.t.signal(pulse, in_idx) {
                m.outs
                    .iter()
                    .for_each(|&out| q.push_back((pulse, idx, out)))
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (start, mut modules) = parse(input);
    let (mut lows, mut highs) = (0, 0);
    for _ in 0..1000 {
        push_button(&mut modules, start, |pulse, _, _| match pulse {
            Low => lows += 1,
            High => highs += 1,
        })
    }
    lows * highs
}

pub fn part2(input: &str) -> usize {
    let (start, mut modules) = parse(input);
    let (out, len) = modules
        .iter()
        .enumerate()
        .find_map(|(i, m)| match (&m.t, &m.outs[..]) {
            (Conjunction(_, len), [m]) if *m == modules.len() => Some((i, *len as usize)),
            _ => None,
        })
        .unwrap();
    let mut cycles = AHashMap::new();
    let mut i: usize = 0;
    while cycles.len() < len {
        i += 1;
        push_button(&mut modules, start, |pulse, in_idx, name| {
            if pulse == High && name == out && !cycles.contains_key(&in_idx) {
                cycles.insert(in_idx, i);
            }
        });
    }
    cycles.into_values().reduce(lcm).unwrap()
}
