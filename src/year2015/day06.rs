use crate::utils::parsers::*;
use ndarray::{s, Array2, ArrayViewMut2};
use Cmd::*;

#[derive(Clone)]
enum Cmd {
    On,
    Toggle,
    Off,
}

fn instr(i: &mut &str) -> PResult<(Cmd, usize, usize, usize, usize)> {
    let cmd = alt((
        "turn on ".value(On),
        "toggle ".value(Toggle),
        "turn off ".value(Off),
    ))
    .parse_next(i)?;
    let ((x0, y0), (x1, y1)) = sep2(coord(usize), "through").parse_next(i)?;
    Ok((cmd, x0, x1, y0, y1))
}

// Use Saturating if it gets included in num_traits::Zero.
fn run_commands(input: &str, f: fn(Cmd, ArrayViewMut2<u32>)) -> u32 {
    let mut grid = Array2::zeros((1000, 1000));
    for line in input.lines() {
        let (cmd, x0, x1, y0, y1) = instr.read(line);
        let slice = grid.slice_mut(s![x0..=x1, y0..=y1]);
        f(cmd, slice);
    }
    grid.sum()
}

pub fn part1(input: &str) -> u32 {
    run_commands(input, |cmd, mut slice| match cmd {
        On => slice |= 1,
        Toggle => slice ^= 1,
        Off => slice &= 0,
    })
}

pub fn part2(input: &str) -> u32 {
    run_commands(input, |cmd, mut slice| match cmd {
        On => slice += 1,
        Toggle => slice += 2,
        Off => slice.mapv_inplace(|v| v.saturating_sub(1)),
    })
}
