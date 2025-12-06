use crate::utils::parsers::*;
use crate::utils::*;
use std::ops::{AddAssign, MulAssign};

struct Col(fn(&mut u64, u64), u64);

impl Col {
    fn apply(&mut self, rhs: u64) {
        self.0(&mut self.1, rhs);
    }
}

fn parse(input: &[u8]) -> (Vec<Col>, Vec<Vec<u8>>) {
    let mut lines: Vec<Vec<u8>> = lines(repeat(.., none_of('\n'))).read(input);
    let cols: Vec<_> = lines
        .pop()
        .unwrap()
        .into_iter()
        .filter(|&c| c == b'+' || c == b'*')
        .map(|c| if c == b'+' { Col(u64::add_assign, 0) } else { Col(u64::mul_assign, 1) })
        .collect();
    (cols, lines)
}

pub fn part1(input: &[u8]) -> u64 {
    let (mut cols, lines) = parse(input);
    lines.into_iter().for_each(|line| {
        cols.iter_mut().zip(spaced(u64).read(line.as_slice())).for_each(|(col, n)| col.apply(n))
    });
    cols.into_iter().map(|c| c.1).sum()
}

pub fn part2(input: &[u8]) -> u64 {
    let (mut cols, lines) = parse(input);
    let transposed = transpose(&lines)
        .into_iter()
        .map(|line| unsafe { str::from_utf8_unchecked(&line) }.trim().to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    cols.iter_mut()
        .zip(transposed.split("\n\n"))
        .for_each(|(col, ns)| ns.lines().for_each(|n| col.apply(n.u64())));
    cols.into_iter().map(|c| c.1).sum()
}
