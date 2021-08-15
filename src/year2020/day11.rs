use itertools::iterate;

use crate::utils::Coord;

fn stabilize(s: &str, p2: bool) -> usize {
    let mut grid: Vec<Vec<char>> = s.lines().map(|row| row.chars().collect()).collect();
    let seats: Vec<(Coord<i64>, Vec<Coord<i64>>)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|x| x.1 == &'L')
                .map(|(c, _)| {
                    let st_coord = Coord::new(r as i64, c as i64);
                    let mut vec = Vec::new();
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let drc = Coord::new(dr, dc);
                            let start = st_coord + drc;
                            let pos = iterate(start, |&i| i + drc)
                                .take_while(|coord| {
                                    0 <= coord.x
                                        && coord.x < grid.len() as i64
                                        && 0 <= coord.y
                                        && coord.y < grid[0].len() as i64
                                })
                                .find(|coord| grid[coord.x as usize][coord.y as usize] == 'L');
                            if let Some(coord) = pos {
                                if p2 || {
                                    let x = st_coord - coord;
                                    x.x.abs() <= 1 && x.y.abs() <= 1
                                } {
                                    vec.push(coord);
                                }
                            }
                        }
                    }
                    (st_coord, vec)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let mut changed = true;
    while std::mem::replace(&mut changed, false) {
        let mut grid2 = grid.clone();
        for (coord, adjs) in &seats {
            let r = coord.x as usize;
            let c = coord.y as usize;
            let adjs_occ: u32 = adjs
                .iter()
                .map(|c| (grid[c.x as usize][c.y as usize] == '#') as u32)
                .sum();
            if grid[r][c] == 'L' && adjs_occ == 0 {
                grid2[r][c] = '#';
                changed = true;
            } else if grid[r][c] == '#' && adjs_occ >= (if p2 { 5 } else { 4 }) {
                grid2[r][c] = 'L';
                changed = true;
            }
        }
        grid = grid2;
    }
    grid.iter()
        .map(|row| row.iter().filter(|&x| *x == '#').count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    stabilize(input, false)
}

pub fn part2(input: &str) -> usize {
    stabilize(input, true)
}
