use crate::utils::*;
use itertools::chain;
use safe_arch::*;
use std::cmp::{max, min};
use Dir::*;

#[derive(Clone)]
struct Grid {
    es: [m256i; 160],
}

fn step_west(row: &m256i) -> m256i {
    shr_imm_u64_m256i::<1>(*row)
        | shl_imm_u64_m256i::<63>(shuffle_ai_i64_all_m256i::<0b00_11_10_01>(*row))
}

fn step_east(row: &m256i) -> m256i {
    shl_imm_u64_m256i::<1>(*row)
        | shr_imm_u64_m256i::<63>(shuffle_ai_i64_all_m256i::<0b10_01_00_11>(*row))
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

fn propose(
    dirs: [Dir; 4],
    [nw, n, ne]: &[m256i; 3],
    [w, c, e]: &[m256i; 3],
    [sw, s, se]: &[m256i; 3],
) -> [m256i; 4] {
    let mut proposals = [*c; 4];
    let mut passed = *nw | *n | *ne | *w | *e | *sw | *s | *se;
    for d in dirs {
        let (row, dir) = match d {
            North => (&mut proposals[0], !(*ne | *n | *nw)),
            South => (&mut proposals[1], !(*se | *s | *sw)),
            West => (&mut proposals[2], !(*nw | *w | *sw)),
            East => (&mut proposals[3], !(*ne | *e | *se)),
        };
        *row &= dir & passed;
        passed &= !dir;
    }
    proposals
}

fn check_collisions(
    [_, s, _, _]: &[m256i; 4],
    [_, _, w, e]: &[m256i; 4],
    [n, _, _, _]: &[m256i; 4],
) -> [m256i; 4] {
    [
        *n & !*s,
        *s & !*n,
        step_west(w) & !step_east(e),
        step_east(e) & !step_west(w),
    ]
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut res = Self {
            es: [Default::default(); 160],
        };
        for (line, r) in input.lines().zip(24..) {
            let mut row = [0_u64; 4];
            for (v, c) in line.chars().zip(72..) {
                row[c / 64] |= ((v == '#') as u64) << (c % 64);
            }
            res.es[r] = m256i::from(row);
        }
        res
    }

    fn step(&mut self, dirs: &mut [Dir; 4]) -> bool {
        let mut next = self.clone();
        let mut moved = false;
        let zeroes = [Default::default(); 2];
        chain!(&zeroes, &self.es, &zeroes)
            .map(|row| [step_east(row), *row, step_west(row)])
            .map_windows(|[above, cur, below]| propose(*dirs, above, cur, below))
            .map_windows(|[above, cur, below]| check_collisions(above, cur, below))
            .enumerate()
            .for_each(|(i, [from_s, from_n, from_e, from_w])| {
                let dests = from_n | from_s | from_w | from_e;
                if dests == m256i::from([0_u64; 4]) {
                    return;
                }
                moved = true;
                next.es[i + 1] &= !from_s;
                next.es[i - 1] &= !from_n;
                next.es[i] &= !step_west(&from_w) & !step_east(&from_e);
                next.es[i] |= dests;
            });
        dirs.rotate_left(1);
        *self = next;
        moved
    }
}

pub fn part1(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let mut dirs = [North, South, West, East];
    for _ in 0..10 {
        grid.step(&mut dirs);
    }
    let (mut min_x, mut min_y) = (u32::MAX, u32::MAX);
    let (mut max_x, mut max_y) = (u32::MIN, u32::MIN);
    let mut elf_cnt = 0;
    for (r, row) in grid.es.into_iter().enumerate() {
        let ns: [u64; 4] = row.into();
        if ns == [0; 4] {
            continue;
        }
        min_y = min(min_y, r as u32);
        max_y = max(max_y, r as u32 + 1);
        let mut gen = ns.iter().rev().enumerate().filter(|p| *p.1 != 0).peekable();
        let (i, n) = gen.peek().unwrap();
        min_x = min(min_x, 64 * *i as u32 + n.leading_zeros());
        let (i, n) = gen.last().unwrap();
        max_x = max(max_x, 64 * i as u32 + 64 - n.trailing_zeros());
        elf_cnt += ns.into_iter().map(|n| n.count_ones()).sum::<u32>();
    }
    (max_x - min_x) * (max_y - min_y) - elf_cnt
}

pub fn part2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut dirs = [North, South, West, East];
    let mut i = 1;
    while grid.step(&mut dirs) {
        i += 1;
    }
    i
}
