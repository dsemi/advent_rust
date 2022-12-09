use crate::utils::Coord;
use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;

fn sim_rope(input: &str, rope_len: usize) -> usize {
    let mut knots = vec![Coord::new(0_i32, 0_i32); rope_len];
    let mut tail_pos = AHashSet::new();
    tail_pos.insert(knots[0]);
    for line in input.lines() {
        let (dir, n) = scanf!(line, "{} {}", char, usize).unwrap();
        for _ in 0..n {
            match dir {
                'L' => knots[0].x -= 1,
                'R' => knots[0].x += 1,
                'U' => knots[0].y += 1,
                'D' => knots[0].y -= 1,
                _ => unreachable!(),
            }
            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    knots[i] += diff.signum();
                }
            }
            tail_pos.insert(*knots.last().unwrap());
        }
    }
    tail_pos.len()
}

pub fn part1(input: &str) -> usize {
    sim_rope(input, 2)
}

pub fn part2(input: &str) -> usize {
    sim_rope(input, 10)
}
