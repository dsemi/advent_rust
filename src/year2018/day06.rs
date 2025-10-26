use crate::utils::parsers::*;
use crate::utils::*;
use Tile::*;
use hashbrown::HashSet;
use std::cmp::{max, min};
use std::collections::VecDeque;

struct Pos {
    id: usize,
    dist: usize,
    pos: C<usize>,
}

#[derive(Clone)]
enum Tile {
    Empty,
    Tied,
    Taken((usize, usize)),
}

fn bounding_box(xs: &[C<i32>]) -> (C<usize>, C<usize>) {
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for &C(x, y) in xs {
        x0 = min(x0, x as usize);
        y0 = min(y0, y as usize);
        x1 = max(x1, x as usize);
        y1 = max(y1, y as usize);
    }
    (C(x0, y0), C(x1 + 1, y1 + 1))
}

pub fn part1(input: &str) -> Option<usize> {
    let coords = lines(c(i32)).read(input);
    let (minp, mut maxp) = bounding_box(&coords);
    maxp -= minp;
    let mut grid = vec![vec![Empty; maxp.0]; maxp.1];
    let mut areas = vec![0; coords.len()];
    let mut frontier = coords
        .into_iter()
        .enumerate()
        .map(|(i, c)| Pos { id: i, dist: 0, pos: C(c.0 as usize - minp.0, c.1 as usize - minp.1) })
        .collect::<VecDeque<_>>();
    while let Some(p) = frontier.pop_front() {
        match grid[p.pos.1][p.pos.0] {
            Taken((id, d)) => {
                if d == p.dist && id != p.id {
                    areas[id] -= 1;
                    grid[p.pos.1][p.pos.0] = Tied;
                }
            }
            Tied => (),
            Empty => {
                areas[p.id] += 1;
                grid[p.pos.1][p.pos.0] = Taken((p.id, p.dist));
                if p.pos.0 > 0 {
                    frontier.push_back(Pos { id: p.id, dist: p.dist + 1, pos: p.pos - C(1, 0) });
                }
                if p.pos.1 > 0 {
                    frontier.push_back(Pos { id: p.id, dist: p.dist + 1, pos: p.pos - C(0, 1) });
                }
                if p.pos.0 < maxp.0 - 1 {
                    frontier.push_back(Pos { id: p.id, dist: p.dist + 1, pos: p.pos + C(1, 0) });
                }
                if p.pos.1 < maxp.1 - 1 {
                    frontier.push_back(Pos { id: p.id, dist: p.dist + 1, pos: p.pos + C(0, 1) });
                }
            }
        }
    }
    for (x, y) in (0..maxp.0).flat_map(|x| [(x, 0), (x, maxp.1 - 1)]) {
        if let Taken((id, _)) = grid[y][x] {
            areas[id] = 0;
        }
    }
    for (x, y) in (0..maxp.1).flat_map(|y| [(0, y), (maxp.0 - 1, y)]) {
        if let Taken((id, _)) = grid[y][x] {
            areas[id] = 0;
        }
    }

    areas.into_iter().max()
}

pub fn part2(input: &str) -> usize {
    const N: i32 = 10_000;
    let coords = lines(c(i32)).read(input);
    let avg_coord = C(
        coords.iter().map(|c| c.0).sum::<i32>() / coords.len() as i32,
        coords.iter().map(|c| c.1).sum::<i32>() / coords.len() as i32,
    );
    let mut region_size = 0;
    let mut frontier = vec![avg_coord].into_iter().collect::<VecDeque<_>>();
    let mut visited = HashSet::new();
    while let Some(p) = frontier.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        if coords.iter().map(|c| p.dist(c)).sum::<i32>() < N {
            region_size += 1;
            frontier.push_back(p - C(1, 0));
            frontier.push_back(p - C(0, 1));
            frontier.push_back(p + C(1, 0));
            frontier.push_back(p + C(0, 1));
        }
    }
    region_size
}
