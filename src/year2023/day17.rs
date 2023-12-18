use crate::utils::*;
use std::iter::repeat;
use std::ops::{AddAssign, SubAssign};

struct AStar {
    width: usize,
    height: usize,
    grid: Vec<Vec<u32>>,
}

impl AStar {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.bytes().map(|x| (x - b'0') as u32).collect())
            .collect();
        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn heur(&self, C(r, c): C<usize>, cost: u32) -> usize {
        (cost as usize + self.height - r + self.width - c) % 128
    }

    fn add_range<const LO: usize, const HI: usize>(
        &self,
        q: &mut [Vec<(C<usize>, bool)>],
        g_score: &mut [Vec<[u32; 2]>],
        pos: C<usize>,
        horz: bool,
        cost: u32,
        f: fn(&mut C<usize>, C<usize>),
    ) {
        repeat(if horz { C(0, 1) } else { C(1, 0) })
            .take(HI)
            .scan((pos, cost), |acc, d| {
                f(&mut acc.0, d);
                acc.1 += self.grid.get_cell(acc.0)?;
                Some(*acc)
            })
            .skip(LO - 1)
            .for_each(|(pos, cost)| {
                let e = &mut g_score[pos][!horz as usize];
                if *e == 0 || cost < *e {
                    q[self.heur(pos, cost)].push((pos, !horz));
                    *e = cost;
                }
            })
    }

    fn a_star<const LO: usize, const HI: usize>(&self) -> u32 {
        let w = self.width;
        let h = self.height;
        let goal = C(h - 1, w - 1);
        let mut g_score: Vec<Vec<[u32; 2]>> = vec![vec![[0u32; 2]; w]; h];
        let mut q = vec![vec![]; 128];
        q[0].push((C(0, 0), false));
        q[0].push((C(0, 0), true));
        for qi in 0.. {
            while let Some((pos, horz)) = q[qi % 128].pop() {
                let pos: C<usize> = pos;
                let cost = g_score[pos][horz as usize];
                if pos == goal {
                    return cost;
                }
                self.add_range::<LO, HI>(&mut q, &mut g_score, pos, horz, cost, C::sub_assign);
                self.add_range::<LO, HI>(&mut q, &mut g_score, pos, horz, cost, C::add_assign);
            }
        }
        unreachable!()
    }
}

pub fn part1(input: &str) -> u32 {
    AStar::parse(input).a_star::<1, 3>()
}

pub fn part2(input: &str) -> u32 {
    AStar::parse(input).a_star::<4, 10>()
}
