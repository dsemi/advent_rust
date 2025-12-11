use crate::utils::parsers::*;
use crate::utils::*;
use good_lp::*;
use rayon::prelude::*;
use streaming_iterator::StreamingIterator;

struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

fn machine(i: &mut &str) -> Result<Machine> {
    let target = repeat(.., alt(('.'.value(0), '#'.value(1))));
    let target: Vec<_> = delimited('[', target, "] ").parse_next(i)?;
    let target = target.into_iter().rev().fold(0, |acc, n| (acc << 1) | n);
    let buttons = spaced(delimited('(', list(u16), ')')).parse_next(i)?;
    let buttons = buttons.iter().map(|ns| ns.iter().fold(0, |acc, n| acc | (1 << n))).collect();
    let joltage = delimited('{', list(u16), '}').parse_next(i)?;
    Ok(Machine { target, buttons, joltage })
}

pub fn part1(input: &str) -> Option<usize> {
    input
        .par_lines()
        .map(|line| machine.read(line))
        .map(|m| (1..).find(|&sz| m.buttons.combinations(sz).any(|c| c.bitxor() == m.target)))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .par_lines()
        .map(|line| {
            let m = machine.read(line);
            let mut vars = ProblemVariables::new();
            let mut presses = Vec::with_capacity(m.buttons.len());
            for &bs in m.buttons.iter() {
                let max = bits(bs).map(|b| m.joltage[b]).min().unwrap();
                presses.push(vars.add(variable().integer().bounds(0..=max)));
            }
            let mut problem = vars.minimise(presses.iter().sum::<Expression>()).using(highs);
            let mut exps = vec![Expression::with_capacity(m.buttons.len()); m.joltage.len()];
            for (&bs, press) in m.buttons.iter().zip(&presses) {
                bits(bs).for_each(|b| exps[b] += press)
            }
            for (exp, jolt) in exps.into_iter().zip(m.joltage) {
                problem.add_constraint(exp.eq(jolt));
            }
            let sol = problem.solve().unwrap();
            presses.into_iter().map(|v| sol.value(v)).sum::<f64>() as usize
        })
        .sum()
}
