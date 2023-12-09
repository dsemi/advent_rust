use crate::utils::parsers::*;
use std::cmp::max;

fn parse(i: &mut &str) -> PResult<(i64, i64, i64, i64)> {
    let (_, x0, _, x1, _, y0, _, y1) =
        ("target area: x=", i64, "..", i64, ", y=", i64, "..", i64).parse_next(i)?;
    Ok((x0, x1, y0, y1))
}

fn hits_target(x0: i64, x1: i64, y0: i64, y1: i64, mut vx: i64, mut vy: i64) -> bool {
    itertools::unfold((0, 0), |p| {
        p.0 += vx;
        p.1 += vy;
        vx = max(0, vx - 1);
        vy -= 1;
        (p.0 <= x1 && p.1 >= y0).then_some(*p)
    })
    .any(|(x, y)| x0 <= x && x <= x1 && y0 <= y && y <= y1)
}

pub fn part1(input: &str) -> i64 {
    let (_, _, y0, _) = parse.read(input);
    y0 * (y0 + 1) / 2
}

pub fn part2(input: &str) -> usize {
    let (x0, x1, y0, y1) = parse.read(input);
    assert!(x0 > 0 && x1 > 0);
    assert!(y0 < 0 && y1 < 0);
    // First triangular number > x0 is lower bound.
    // n^2 + n - 2x0 = 0
    let mx = ((1.0 + 8.0 * x0 as f64).sqrt() / 2.0 - 0.5).ceil() as i64;
    (mx..=x1)
        .flat_map(|x| (y0..=-y0).filter(move |y| hits_target(x0, x1, y0, y1, x, *y)))
        .count()
}
