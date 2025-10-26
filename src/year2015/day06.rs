use crate::utils::parsers::*;
use crate::utils::{Interval, Rect};
use ndarray::{s, Array2, ArrayViewMut2, Zip};
use Cmd::*;

#[derive(Clone)]
enum Cmd {
    On,
    Toggle,
    Off,
}

struct Instr {
    cmd: Cmd,
    rect: Rect<usize>,
}

fn instr(i: &mut &str) -> ModalResult<Instr> {
    let cmd = alt(("turn on ".value(On), "toggle ".value(Toggle), "turn off ".value(Off)))
        .parse_next(i)?;
    let ((x0, y0), (x1, y1)) = sep2(coord(usize), "through").parse_next(i)?;
    let rect = Rect::new(x0, x1 + 1, y0, y1 + 1);
    Ok(Instr { cmd, rect })
}

// Use Saturating<u32> if it gets included in num::Zero.
fn run_commands(input: &str, f: fn(Cmd, ArrayViewMut2<u32>)) -> u32 {
    let instrs: Vec<_> = input.lines().map(|line| instr.read(line)).collect();
    let (mut ls, mut ws) = (vec![], vec![]);
    instrs.iter().for_each(|instr| {
        ls.push(instr.rect.l.lo);
        ls.push(instr.rect.l.hi);
        ws.push(instr.rect.w.lo);
        ws.push(instr.rect.w.hi);
    });
    ls.sort_unstable();
    ls.dedup();
    ws.sort_unstable();
    ws.dedup();
    let mut l_idxs = vec![0; 1001];
    for (i, &l) in ls.iter().enumerate() {
        l_idxs[l] = i;
    }
    let mut w_idxs = vec![0; 1001];
    for (i, &w) in ws.iter().enumerate() {
        w_idxs[w] = i;
    }
    let rects: Vec<_> = ls
        .iter()
        .zip(ls.iter().skip(1))
        .flat_map(|(&l_lo, &l_hi)| {
            ws.iter()
                .zip(ws.iter().skip(1))
                .map(move |(&w_lo, &w_hi)| Rect::new(l_lo, l_hi, w_lo, w_hi))
        })
        .collect();
    let rects = Array2::from_shape_vec((ls.len() - 1, ws.len() - 1), rects).unwrap();
    let mut grid = Array2::zeros(rects.raw_dim());
    for instr in instrs {
        let ls = Interval::new(l_idxs[instr.rect.l.lo], l_idxs[instr.rect.l.hi]);
        let ws = Interval::new(w_idxs[instr.rect.w.lo], w_idxs[instr.rect.w.hi]);
        let slice = grid.slice_mut(s![ls.range(), ws.range()]);
        f(instr.cmd, slice);
    }
    Zip::from(&rects).and(&grid).fold(0, |acc, rect, v| acc + rect.area() as u32 * v)
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
