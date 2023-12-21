use crate::utils::*;

fn neighbors(grid: &[Vec<u8>], pos: C<usize>) -> impl Iterator<Item = C<usize>> + '_ {
    [pos - C(1, 0), pos + C(1, 0), pos - C(0, 1), pos + C(0, 1)]
        .into_iter()
        .filter(|&p| matches!(grid.get_cell(p), Some(b'.' | b'S')))
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    bfs(C(grid.len() / 2, grid.len() / 2), |n| neighbors(&grid, *n))
        .take_while(|&(d, _)| d <= 64)
        .filter(|&(d, _)| d & 1 == 0)
        .count()
}

const GOAL: usize = 26501365;

pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    let len = grid.len();
    let start = C(len / 2, len / 2);
    let (mut evens, mut odds) = (0, 0);
    let (mut outer_evens, mut outer_odds) = (0, 0);
    for (d, p) in bfs(start, |n| neighbors(&grid, *n)) {
        if d & 1 == 0 {
            evens += 1;
            outer_evens += (p.dist(&start) > 65) as usize;
        } else {
            odds += 1;
            outer_odds += (p.dist(&start) > 65) as usize;
        }
    }
    let a0 = odds - outer_odds;
    let a1 = evens + outer_evens + 4 * odds - 2 * outer_odds;
    let a2 = 9 * odds - 3 * outer_odds + 4 * evens + 2 * outer_evens;
    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;
    let n = GOAL / len;
    b0 + b1 * n + (b2 - b1) * (n * (n - 1) / 2)
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
