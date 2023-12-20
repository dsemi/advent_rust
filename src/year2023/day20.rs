use crate::utils::parsers::*;
use crate::utils::*;
use ahash::AHashMap;
use num::integer::lcm;
use std::collections::VecDeque;
use Pulse::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Default)]
struct Broadcast {
    outs: Vec<(usize, usize)>,
}

#[derive(Debug, Default)]
struct FlipFlop {
    on: bool,
    outs: Vec<(usize, usize)>,
}

#[derive(Debug, Default)]
struct Conjunction {
    ins: u64,
    all_on: u64,
    outs: Vec<(usize, usize)>,
}

#[derive(Debug)]
enum Module {
    B(Broadcast),
    F(FlipFlop),
    C(Conjunction),
}

fn parse(input: &str) -> (Vec<Module>, usize) {
    let mut modules = Vec::new();
    let mut ui = UniqueIdx::new();
    let mut input_counts = AHashMap::new();
    input.lines().for_each(|line| {
        if line.starts_with("broadcaster") {
            ui.idx("broadcaster");
            modules.push(Module::B(Default::default()));
        } else if line.starts_with('%') {
            ui.idx(&line[1..3]);
            modules.push(Module::F(Default::default()));
        } else {
            ui.idx(&line[1..3]);
            modules.push(Module::C(Default::default()));
        }
    });
    input.lines().enumerate().for_each(|(i, line)| {
        let outs = preceded((opt(alt(('%', '&'))), alpha1, " -> "), list(alpha1)).read(line);
        let outputs = match modules.get_mut(i).unwrap() {
            Module::B(Broadcast { outs }) => outs,
            Module::F(FlipFlop { outs, .. }) => outs,
            Module::C(Conjunction { outs, .. }) => outs,
        };
        for out in outs {
            let k = ui.idx(out);
            let e = input_counts.entry(k).or_default();
            outputs.push((*e, k));
            *e += 1;
        }
    });
    modules.iter_mut().enumerate().for_each(|(i, module)| {
        if let Module::C(Conjunction { all_on, .. }) = module {
            *all_on = (1 << input_counts[&i]) - 1;
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
            Some(Module::B(Broadcast { outs })) => outs
                .iter()
                .for_each(|&(idx, out)| q.push_back((pulse, idx, out))),
            Some(Module::F(FlipFlop { on, outs })) => {
                if pulse == Low {
                    *on = !*on;
                    let pulse = if *on { High } else { Low };
                    outs.iter()
                        .for_each(|&(idx, out)| q.push_back((pulse, idx, out)));
                }
            }
            Some(Module::C(Conjunction { ins, all_on, outs })) => {
                if pulse == Low {
                    *ins &= !(1 << in_idx);
                } else {
                    *ins |= 1 << in_idx;
                }
                let pulse = if *ins == *all_on { Low } else { High };
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
            Module::C(Conjunction { outs, all_on, .. }) if *outs == vec![(0, modules.len())] => {
                Some((i, all_on.count_ones() as usize))
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
