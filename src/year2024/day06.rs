use itertools::{iterate, Itertools};
use rayon::prelude::*;
use std::iter::successors;

use crate::utils::*;

pub fn path(grid: &Grid<u8, i32>) -> impl Iterator<Item = (C<i32>, C<i32>)> + use<'_> {
    let mut seen = grid.same_size_with(false);
    successors(Some((grid.position(|&v| v == b'^').unwrap(), C(-1, 0))), |(mut pos, mut dir)| {
        dir = iterate(dir, |dir| dir * C(0, -1))
            .find(|dir| !matches!(grid.get(pos + dir), Some(b'#')))?;
        pos += dir;
        grid.in_bounds(pos).then_some((pos, dir))
    })
    .filter(move |&(pos, _)| !std::mem::replace(&mut seen[pos], true))
}

pub fn part1(input: &str) -> usize {
    path(&input.bytes().collect()).count()
}

fn populate(g: &Grid<u8, i32>, jmp: &mut Grid<C<i32>, i32>, st: C<i32>, f: fn(&C<i32>) -> C<i32>) {
    iterate(f(&st), f)
        .scan(st, |skip, pos| {
            if *g.get(pos)? == b'#' {
                *skip = f(&pos);
            }
            Some((pos, *skip))
        })
        .for_each(|(pos, v)| jmp[pos] = v);
}

fn jumps(grid: &Grid<u8, i32>) -> [Grid<C<i32>, i32>; 4] {
    let mut res = std::array::from_fn(|_| grid.same_size_with(C(0, 0)));
    for col in 0..grid.cols {
        populate(grid, &mut res[0], C(-1, col), |pos| pos + C(1, 0));
        populate(grid, &mut res[1], C(grid.rows, col), |pos| pos - C(1, 0));
    }
    for row in 0..grid.rows {
        populate(grid, &mut res[2], C(row, -1), |pos| pos + C(0, 1));
        populate(grid, &mut res[3], C(row, grid.cols), |pos| pos - C(0, 1));
    }
    res
}

fn idx(C(a, b): C<i32>) -> usize {
    (((a as u8 & 3).div_ceil(2) << 2) | (b as u8 & 3).div_ceil(2)).leading_zeros() as usize - 4
}

fn loops(jmps: &[Grid<C<i32>, i32>], b: C<i32>, mut pos: C<i32>, mut dir: C<i32>) -> bool {
    let mut seen = jmps[0].same_size_with(0_u8);
    while jmps[0].in_bounds(pos) {
        let bit = idx(dir);
        if seen[pos] & (1 << bit) != 0 {
            return true;
        }
        seen[pos] |= 1 << bit;
        let next = jmps[bit][pos];
        pos = match dir {
            C(-1, 0) if pos.1 == b.1 && (next.0..pos.0).contains(&b.0) => b - dir,
            C(1, 0) if pos.1 == b.1 && (pos.0..=next.0).contains(&b.0) => b - dir,
            C(0, -1) if pos.0 == b.0 && (next.1..pos.1).contains(&b.1) => b - dir,
            C(0, 1) if pos.0 == b.0 && (pos.1..=next.1).contains(&b.1) => b - dir,
            _ => next,
        };
        dir *= C(0, -1);
    }
    false
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let jmps = jumps(&grid);
    let candidates: Vec<_> = path(&grid).tuple_windows().collect();
    candidates.into_par_iter().filter(|&((pos, dir), (b, _))| loops(&jmps, b, pos, dir)).count()
}
