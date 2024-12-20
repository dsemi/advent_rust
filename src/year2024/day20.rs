use crate::utils::*;
use rayon::prelude::*;

fn solve(input: &str, cheat_time: i32) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let mut prev = grid.position(|&v| v == b'S').unwrap();
    let mut times = grid.same_size_with(0);
    let path: Vec<_> = std::iter::successors(Some(prev), |&p| {
        let prev = std::mem::replace(&mut prev, p);
        [C(p.0, p.1 - 1), C(p.0, p.1 + 1), C(p.0 + 1, p.1), C(p.0 - 1, p.1)]
            .into_iter()
            .find(|&pos| pos != prev && grid[pos] != b'#')
    })
    .collect();
    path.iter().zip(1..).for_each(|(&p, time)| times[p] = time);
    path[..path.len() - 101]
        .par_iter()
        .enumerate()
        .map(|(t, p)| {
            let t = t as i32 + 1;
            let f = |d| (*times.get(p + d).unwrap_or(&0) >= t + C(0, 0).dist(&d) + 100) as usize;
            (1..=cheat_time)
                .map(|r| {
                    let n = f(C(r, 0)) + f(C(-r, 0)) + f(C(0, r)) + f(C(0, -r));
                    n + (1..=cheat_time - r)
                        .map(|c| f(C(r, c)) + f(C(r, -c)) + f(C(-r, c)) + f(C(-r, -c)))
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 20)
}
