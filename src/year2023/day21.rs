use crate::utils::*;
use std::collections::VecDeque;

fn dists(grid: Grid<u8>) -> Grid<Option<usize>> {
    let start = C(grid.rows / 2, grid.cols / 2);
    let mut frontier = VecDeque::new();
    frontier.push_back((start, 0));
    let mut visited: Grid<Option<usize>> = grid.same_size();
    visited[start] = Some(0);
    while let Some((pos, d)) = frontier.pop_front() {
        for p in [pos - C(1, 0), pos + C(1, 0), pos - C(0, 1), pos + C(0, 1)] {
            if matches!(grid.get(p), Some(b'.' | b'S')) && visited[p].is_none() {
                frontier.push_back((p, d + 1));
                visited[p] = Some(d + 1);
            }
        }
    }
    visited
}

pub fn part1(input: &str) -> usize {
    let dists = dists(input.bytes().collect());
    dists
        .into_iter()
        .filter(|&d| matches!(d, Some(n) if n <= 64 && n & 1 == 0))
        .count()
}

const GOAL: usize = 26501365;

pub fn part2(input: &str) -> usize {
    let dists = dists(input.bytes().collect());
    let start = C(dists.rows / 2, dists.cols / 2);
    let (mut evens, mut odds) = (0, 0);
    let (mut outer_evens, mut outer_odds) = (0, 0);
    dists
        .idx_iter()
        .filter_map(|(p, d)| d.is_some().then_some(p))
        .for_each(|p| {
            let d = p.dist(&start);
            if d & 1 == 0 {
                evens += 1;
                outer_evens += (d > 65) as usize;
            } else {
                odds += 1;
                outer_odds += (d > 65) as usize;
            }
        });
    let n = GOAL / dists.rows;
    (n + 1) * (n + 1) * odds - (n + 1) * outer_odds + n * n * evens + n * outer_evens
}

//
//  o
// ooo
//  o

// odds - outer_odds

//      o
//     ooo
//   x ooo x

//  oo xxx oo
// ooo xxx ooo
//  oo xxx oo

//   x ooo x
//     ooo
//      o

// evens + outer_evens +
// 4*odds - 2*outer_odds

//          o
//         ooo
//       x ooo x

//      oo xxx oo
//     ooo xxx ooo
//   x ooo xxx ooo x

//  oo xxx ooo xxx oo
// ooo xxx ooo xxx ooo
//  oo xxx ooo xxx oo

//   x ooo xxx ooo x
//     ooo xxx ooo
//      oo xxx oo

//       x ooo x
//         ooo
//          o

// 9*odds - 3*outer_odds +
// 4*evens + 2*outer_evens
