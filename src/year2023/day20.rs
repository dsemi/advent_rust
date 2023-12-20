use crate::utils::parsers::*;
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

#[derive(Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(u64, u32),
}

struct Module(ModuleType, Vec<(usize, usize)>);

fn parse(input: &str) -> (Vec<Module>, usize) {
    let mut modules = Vec::new();
    let mut ui = UniqueIdx::new();
    let mut input_counts = AHashMap::new();
    let mut m_outs = Vec::new();
    input.lines().for_each(|line| {
        let (t, name, outs) = (
            opt(alt((
                '%'.value(FlipFlop(false)),
                '&'.value(Conjunction(0, 0)),
            )))
            .map(|x| x.unwrap_or(Broadcast)),
            alpha1,
            preceded(" -> ", list(alpha1)),
        )
            .read(line);
        modules.push(Module(t, Vec::new()));
        ui.idx(name);
        m_outs.push(outs);
    });
    m_outs.into_iter().enumerate().for_each(|(i, outs)| {
        outs.into_iter().for_each(|out| {
            let k = ui.idx(out);
            let e = input_counts.entry(k).or_default();
            modules[i].1.push((*e, k));
            *e += 1;
        });
    });
    modules.iter_mut().enumerate().for_each(|(i, module)| {
        if let Module(Conjunction(_, cnt), _) = module {
            *cnt = input_counts[&i] as u32;
        }
    });
    (modules, ui.idx("broadcaster"))
}

fn push_button(modules: &mut [Module], start: usize, mut f: impl FnMut(Pulse, usize, usize)) {
    let mut q = VecDeque::new();
    q.push_back((Low, 0, start));
    while let Some((pulse, in_idx, idx)) = q.pop_front() {
        f(pulse, in_idx, idx);
        match modules.get_mut(idx) {
            Some(Module(Broadcast, outs)) => outs
                .iter()
                .for_each(|&(idx, out)| q.push_back((pulse, idx, out))),
            Some(Module(FlipFlop(on), outs)) => {
                if pulse == Low {
                    *on = !*on;
                    let pulse = if *on { High } else { Low };
                    outs.iter()
                        .for_each(|&(idx, out)| q.push_back((pulse, idx, out)));
                }
            }
            Some(Module(Conjunction(ins, len), outs)) => {
                if pulse == Low {
                    *ins &= !(1 << in_idx);
                } else {
                    *ins |= 1 << in_idx;
                }
                let pulse = if ins.count_ones() == *len { Low } else { High };
                outs.iter()
                    .for_each(|&(idx, out)| q.push_back((pulse, idx, out)));
            }
            None => (),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (mut modules, start) = parse(input);
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
    let (mut modules, start) = parse(input);
    let (out, len) = modules
        .iter()
        .enumerate()
        .find_map(|(i, m)| match m {
            Module(Conjunction(_, len), outs) if *outs == vec![(0, modules.len())] => {
                Some((i, *len as usize))
            }
            _ => None,
        })
        .unwrap();
    let mut cycles = vec![0; len];
    let mut i: usize = 0;
    while cycles.iter().any(|&c| c == 0) {
        i += 1;
        push_button(&mut modules, start, |pulse, in_idx, name| {
            if pulse == High && name == out && cycles[in_idx] == 0 {
                cycles[in_idx] = i;
            }
        });
    }
    cycles.into_iter().reduce(lcm).unwrap()
}
