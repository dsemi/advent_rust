use crate::utils::*;
use ahash::AHashSet;
use std::cmp::{max, min};
use std::collections::VecDeque;
use Tile::*;

fn parse_coords(input: &str) -> Vec<Coord<i32>> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(", ").unwrap();
            Coord::new(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

struct Pos {
    id: usize,
    dist: usize,
    pos: Coord<usize>,
}

#[derive(Clone)]
enum Tile {
    Empty,
    Tied,
    Taken((usize, usize)),
}

fn bounding_box(xs: &[Coord<i32>]) -> (Coord<usize>, Coord<usize>) {
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for x in xs {
        x0 = min(x0, x.x as usize);
        y0 = min(y0, x.y as usize);
        x1 = max(x1, x.x as usize);
        y1 = max(y1, x.y as usize);
    }
    (Coord::new(x0, y0), Coord::new(x1 + 1, y1 + 1))
}

pub fn part1(input: &str) -> Option<usize> {
    let coords = parse_coords(input);
    let (minp, mut maxp) = bounding_box(&coords);
    maxp -= minp;
    let mut grid = vec![vec![Empty; maxp.x]; maxp.y];
    let mut areas = vec![0; coords.len()];
    let mut frontier = coords
        .into_iter()
        .enumerate()
        .map(|(i, c)| Pos {
            id: i,
            dist: 0,
            pos: Coord::new(c.x as usize - minp.x, c.y as usize - minp.y),
        })
        .collect::<VecDeque<_>>();
    while let Some(p) = frontier.pop_front() {
        match grid[p.pos.y][p.pos.x] {
            Taken((id, d)) => {
                if d == p.dist && id != p.id {
                    areas[id] -= 1;
                    grid[p.pos.y][p.pos.x] = Tied;
                }
            }
            Tied => (),
            Empty => {
                areas[p.id] += 1;
                grid[p.pos.y][p.pos.x] = Taken((p.id, p.dist));
                if p.pos.x > 0 {
                    frontier.push_back(Pos {
                        id: p.id,
                        dist: p.dist + 1,
                        pos: p.pos - Coord::new(1, 0),
                    });
                }
                if p.pos.y > 0 {
                    frontier.push_back(Pos {
                        id: p.id,
                        dist: p.dist + 1,
                        pos: p.pos - Coord::new(0, 1),
                    });
                }
                if p.pos.x < maxp.x - 1 {
                    frontier.push_back(Pos {
                        id: p.id,
                        dist: p.dist + 1,
                        pos: p.pos + Coord::new(1, 0),
                    });
                }
                if p.pos.y < maxp.y - 1 {
                    frontier.push_back(Pos {
                        id: p.id,
                        dist: p.dist + 1,
                        pos: p.pos + Coord::new(0, 1),
                    });
                }
            }
        }
    }
    for (x, y) in (0..maxp.x).flat_map(|x| [(x, 0), (x, maxp.y - 1)]) {
        if let Taken((id, _)) = grid[y][x] {
            areas[id] = 0;
        }
    }
    for (x, y) in (0..maxp.y).flat_map(|y| [(0, y), (maxp.x - 1, y)]) {
        if let Taken((id, _)) = grid[y][x] {
            areas[id] = 0;
        }
    }

    areas.into_iter().max()
}

pub fn part2(input: &str) -> usize {
    let n = 10_000;
    let coords = parse_coords(input);
    let avg_coord = Coord::new(
        coords.iter().map(|c| c.x).sum::<i32>() / coords.len() as i32,
        coords.iter().map(|c| c.y).sum::<i32>() / coords.len() as i32,
    );
    let mut region_size = 0;
    let mut frontier = vec![avg_coord].into_iter().collect::<VecDeque<_>>();
    let mut visited = AHashSet::new();
    while let Some(p) = frontier.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        if coords.iter().map(|c| dist(&p, c)).sum::<i32>() < n {
            region_size += 1;
            frontier.push_back(p - Coord::new(1, 0));
            frontier.push_back(p - Coord::new(0, 1));
            frontier.push_back(p + Coord::new(1, 0));
            frontier.push_back(p + Coord::new(0, 1));
        }
    }
    region_size
}
