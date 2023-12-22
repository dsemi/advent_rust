use crate::utils::*;
use std::iter::repeat;
use std::ops::{AddAssign, SubAssign};

struct AStar(Grid<u32>);

impl AStar {
    fn parse(input: &str) -> Self {
        Self(Grid::ints(input.bytes()))
    }

    fn heur(&self, C(r, c): C<usize>, cost: u32) -> usize {
        (cost as usize + self.0.rows - r + self.0.cols - c) % 128
    }

    fn add_range<const LO: usize, const HI: usize>(
        &self,
        q: &mut [Vec<(C<usize>, bool)>],
        g_score: &mut Grid<[u32; 2]>,
        pos: C<usize>,
        horz: bool,
        cost: u32,
        f: fn(&mut C<usize>, C<usize>),
    ) {
        repeat(if horz { C(0, 1) } else { C(1, 0) })
            .take(HI)
            .scan((pos, cost), |acc, d| {
                f(&mut acc.0, d);
                acc.1 += self.0.get(acc.0)?;
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
        let w = self.0.cols;
        let h = self.0.rows;
        let goal = C(h - 1, w - 1);
        let mut g_score: Grid<[u32; 2]> = Grid::new(h, w);
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
