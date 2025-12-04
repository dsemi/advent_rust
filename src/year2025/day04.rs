use crate::utils::*;
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

fn remove_accessible(m0: &mut Array2<u16>) -> u16 {
    let kernel = array![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let mut m1 = m0.conv(&kernel, ConvMode::Same, PaddingMode::Zeros).unwrap();
    m1.mapv_inplace(|v| u16::from(v < 4));
    m1 &= m0 as &Array2<u16>;
    let sum = m1.sum();
    *m0 ^= &m1;
    sum
}

pub fn part1(input: &str) -> u16 {
    let grid: Grid<_> = grid_from_iter(input.bytes(), b'\n', |b| u16::from(b == b'@'));
    let mut m = Array::from_shape_vec((grid.rows, grid.cols), grid.elems).unwrap();
    remove_accessible(&mut m)
}

pub fn part2(input: &str) -> u16 {
    let grid: Grid<_> = grid_from_iter(input.bytes(), b'\n', |b| u16::from(b == b'@'));
    let m = Array::from_shape_vec((grid.rows, grid.cols), grid.elems).unwrap();
    (0..).scan(m, |m, _| Some(remove_accessible(m))).take_while(|&n| n > 0).sum()
}
