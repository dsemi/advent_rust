use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

fn max_height(x0: i64, x1: i64, y0: i64, y1: i64, mut vx: i64, mut vy: i64) -> Option<i64> {
    let (mut x, mut y): (i64, i64) = (0, 0);
    let mut max_y = 0;
    while y >= y0 {
        if x0 <= x && x <= x1 && y0 <= y && y <= y1 {
            return Some(max_y);
        }
        x += vx;
        y += vy;
        max_y = max(max_y, y);
        vx = max(0, vx - 1);
        vy -= 1;
    }
    None
}

fn heights(input: &str) -> impl Iterator<Item = i64> {
    let (x0, x1, y0, y1) =
        scanf!(input, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
    assert!(x0 > 0 && x1 > 0);
    assert!(y0 < 0 && y1 < 0);
    // First triangular number > x0 is lower bound.
    // n^2 + n - 2x0 = 0
    let mx = ((1.0 + 8.0 * x0 as f64).sqrt() / 2.0 - 0.5).ceil() as i64;
    (mx..=x1).flat_map(move |x| (y0..=-y0).filter_map(move |y| max_height(x0, x1, y0, y1, x, y)))
}

pub fn part1(input: &str) -> Option<i64> {
    heights(input).max()
}

pub fn part2(input: &str) -> usize {
    heights(input).count()
}
