use crate::year2019::intcode;
use std::cmp::max;

fn is_pulled(prog: &intcode::Program, x: i64, y: i64) -> bool {
    let mut prog = prog.clone();
    prog.input.extend(&[x, y]);
    prog.run();
    prog.output.pop_front().unwrap() == 1
}

pub fn part1(input: &str) -> i64 {
    let prog = intcode::new(input);
    let (mut min_x, mut max_x) = (0, 0);
    (0..50)
        .filter_map(|y| {
            (min_x..50).find(|&x| is_pulled(&prog, x, y)).map(|mx| {
                min_x = mx;
                max_x = (max(min_x, max_x)..50)
                    .find(|&x| !is_pulled(&prog, x, y))
                    .unwrap();
                max_x - min_x
            })
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let prog = intcode::new(input);
    let mut x = 99;
    let mut y = (0..).find(|&y| is_pulled(&prog, x, y)).unwrap();
    while !is_pulled(&prog, x - 99, y + 99) {
        y *= 2;
        x = (x * 2 + 1..).find(|&x| !is_pulled(&prog, x, y)).unwrap() - 1;
    }

    let xs: Vec<i64> = (x / 2..=x).collect();
    let ys: Vec<i64> = (y / 2..=y).collect();
    let i = ys.partition_point(|&y| {
        let i = xs.partition_point(|&x| is_pulled(&prog, x, y)) - 1;
        !is_pulled(&prog, xs[i] - 99, y + 99)
    });
    // Small buffer since fitting square isn't fully ordered.
    y = (ys[i] - 5..)
        .find(|&y| {
            let i = xs.partition_point(|&x| is_pulled(&prog, x, y)) - 1;
            x = xs[i] - 99;
            is_pulled(&prog, x, y + 99)
        })
        .unwrap();

    x * 10000 + y
}
