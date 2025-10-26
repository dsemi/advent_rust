use crate::utils::*;
use Portal::*;
use hashbrown::HashMap;

type Pos = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Portal {
    Outer(Pos),
    Inner(Pos),
}

#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Portal(Portal),
    Start,
    End,
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    moves: HashMap<Pos, Vec<(usize, Pos)>>,
}

fn parse_maze(input: &str) -> (Maze, Pos, Pos) {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut portals: HashMap<String, Vec<Portal>> = HashMap::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c].is_ascii_uppercase() {
                if r > 0 && grid[r - 1][c].is_ascii_uppercase()
                    || c > 0 && grid[r][c - 1].is_ascii_uppercase()
                {
                    continue;
                }
                let (k, b, coord) = if r > 0 && grid[r - 1][c] == '.' {
                    (vec![grid[r][c], grid[r + 1][c]], r + 2 == grid.len(), (r - 1, c))
                } else if c > 0 && grid[r][c - 1] == '.' {
                    (vec![grid[r][c], grid[r][c + 1]], c + 2 == grid[r].len(), (r, c - 1))
                } else if grid[r + 1][c].is_ascii_uppercase() {
                    (vec![grid[r][c], grid[r + 1][c]], r == 0, (r + 2, c))
                } else {
                    (vec![grid[r][c], grid[r][c + 1]], c == 0, (r, c + 2))
                };
                let e = portals.entry(k.into_iter().collect()).or_default();
                e.push(if b { Outer(coord) } else { Inner(coord) });
            }
        }
    }
    let mut bi: HashMap<Portal, Portal> = HashMap::new();
    let mut start = None;
    let mut end = None;
    for (k, ps) in portals.iter() {
        if ps.len() == 2 {
            bi.insert(ps[0], ps[1]);
            bi.insert(ps[1], ps[0]);
        } else if let Outer(p) = ps[0] {
            if k == "AA" {
                start = Some(p);
            } else if k == "ZZ" {
                end = Some(p);
            }
        } else {
            unreachable!();
        }
    }
    let grid2 = grid
        .into_iter()
        .enumerate()
        .map(|(r, row)| {
            row.into_iter()
                .enumerate()
                .map(|(c, v)| {
                    if let Some(portal) = bi.get(&Outer((r, c))) {
                        Tile::Portal(*portal)
                    } else if let Some(portal) = bi.get(&Inner((r, c))) {
                        Tile::Portal(*portal)
                    } else if (r, c) == start.unwrap() {
                        Tile::Start
                    } else if (r, c) == end.unwrap() {
                        Tile::End
                    } else if v == '.' {
                        Tile::Floor
                    } else {
                        Tile::Wall
                    }
                })
                .collect()
        })
        .collect();
    (Maze { grid: grid2, moves: HashMap::new() }, start.unwrap(), end.unwrap())
}

impl Maze {
    fn available_moves(&mut self, pos: &(usize, usize)) -> Vec<(usize, (usize, usize))> {
        if !self.moves.contains_key(pos) {
            let mut moves = Vec::new();
            if let Tile::Portal(portal) = self.grid[pos.0][pos.1] {
                match portal {
                    Outer(p) => moves.push((1, p)),
                    Inner(p) => moves.push((1, p)),
                }
            }
            moves.extend(
                bfs(*pos, |st| {
                    let (r, c) = *st;
                    if st != pos && self.grid[r][c] != Tile::Floor {
                        return vec![];
                    }
                    vec![(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                        .into_iter()
                        .filter(|&(r, c)| self.grid[r][c] != Tile::Wall)
                        .collect()
                })
                .filter(|(_, (r, c))| self.grid[*r][*c] != Tile::Floor),
            );
            self.moves.insert(*pos, moves);
        }
        self.moves[pos].clone()
    }
}

pub fn part1(input: &str) -> Option<usize> {
    let (mut maze, start, end) = parse_maze(input);
    dijkstra(start, |x| maze.available_moves(x)).find_map(|(d, st)| (st == end).then_some(d))
}

pub fn part2(input: &str) -> Option<usize> {
    let (mut maze, start, end) = parse_maze(input);
    dijkstra((start, 0), |(x, depth)| {
        maze.available_moves(x)
            .into_iter()
            .filter_map(|(dist, (r, c))| match maze.grid[r][c] {
                Tile::Portal(Outer(p)) if p == *x => {
                    if *depth == 0 {
                        None
                    } else {
                        Some((dist, ((r, c), depth - 1)))
                    }
                }
                Tile::Portal(Inner(p)) if p == *x => Some((dist, ((r, c), depth + 1))),
                _ => Some((dist, ((r, c), *depth))),
            })
            .collect::<Vec<_>>()
    })
    .find_map(|(d, st)| (st == (end, 0)).then_some(d))
}
