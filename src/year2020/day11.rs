use crate::utils::*;
use itertools::iterate;

fn stabilize(s: &str, p2: bool) -> usize {
    let mut grid: Grid<char> = s.chars().collect();
    let seats: Vec<(C<i64>, Vec<C<i64>>)> = grid
        .idx_iter()
        .filter(|&(_, x)| x == &'L')
        .map(|(C(r, c), _)| {
            let st_coord = C(r as i64, c as i64);
            let mut vec = Vec::new();
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let drc = C(dr, dc);
                    let start = st_coord + drc;
                    let pos = iterate(start, |&i| i + drc)
                        .take_while(|&i| grid.in_bounds(i))
                        .find(|&coord| grid[coord] == 'L');
                    if let Some(coord) = pos {
                        if p2 || {
                            let C(x, y) = st_coord - coord;
                            x.abs() <= 1 && y.abs() <= 1
                        } {
                            vec.push(coord);
                        }
                    }
                }
            }
            (st_coord, vec)
        })
        .collect();
    let mut changed = true;
    while std::mem::replace(&mut changed, false) {
        let mut grid2 = grid.clone();
        for (coord, adjs) in &seats {
            let r = coord.0 as usize;
            let c = coord.1 as usize;
            let adjs_occ: u32 = adjs.iter().map(|&c| (grid[c] == '#') as u32).sum();
            if grid[(r, c)] == 'L' && adjs_occ == 0 {
                grid2[(r, c)] = '#';
                changed = true;
            } else if grid[(r, c)] == '#' && adjs_occ >= (if p2 { 5 } else { 4 }) {
                grid2[(r, c)] = 'L';
                changed = true;
            }
        }
        grid = grid2;
    }
    grid.into_iter().filter(|&x| x == '#').count()
}

pub fn part1(input: &str) -> usize {
    stabilize(input, false)
}

pub fn part2(input: &str) -> usize {
    stabilize(input, true)
}
