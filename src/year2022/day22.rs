use crate::utils::*;
use ahash::AHashMap;
use itertools::iterate;
use num::integer::gcd;
use regex::Regex;
use Face::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Face {
    Top,
    Bottom,
    Back,
    Front,
    Right,
    Left,
}

impl Face {
    fn fall_off(&self, dir: C3<i32>) -> (Face, C3<i32>) {
        (
            match dir {
                C3(0, 0, 1) => Top,
                C3(0, 0, -1) => Bottom,
                C3(0, 1, 0) => Back,
                C3(0, -1, 0) => Front,
                C3(1, 0, 0) => Right,
                C3(-1, 0, 0) => Left,
                _ => unreachable!(),
            },
            match self {
                Top => C3(0, 0, -1),
                Bottom => C3(0, 0, 1),
                Back => C3(0, -1, 0),
                Front => C3(0, 1, 0),
                Right => C3(-1, 0, 0),
                Left => C3(1, 0, 0),
            },
        )
    }

    fn rotate_left(&self, C3(x, y, z): C3<i32>) -> C3<i32> {
        match self {
            Top => C3(-y, x, z),
            Bottom => C3(y, -x, z),
            Back => C3(z, y, -x),
            Front => C3(-z, y, x),
            Right => C3(x, -z, y),
            Left => C3(x, z, -y),
        }
    }

    fn rotate_right(&self, C3(x, y, z): C3<i32>) -> C3<i32> {
        match self {
            Top => C3(y, -x, z),
            Bottom => C3(-y, x, z),
            Back => C3(-z, y, x),
            Front => C3(z, y, -x),
            Right => C3(x, z, -y),
            Left => C3(x, -z, y),
        }
    }
}

enum Instr {
    Turn(bool),
    Step(usize),
}

type Pt = (C<i32>, C<i32>);

struct Board {
    grid: Vec<Vec<char>>,
    path: Vec<Instr>,
    top_left: C<i32>,
}

impl Board {
    fn new(input: &str) -> Self {
        let pts = input.split("\n\n").collect::<Vec<_>>();
        let reg = Regex::new(r"\d+|.").unwrap();
        let grid: Vec<Vec<char>> = pts[0].lines().map(|line| line.chars().collect()).collect();
        Self {
            top_left: C(0, grid[0].iter().position(|&c| c != ' ').unwrap() as i32),
            grid,
            path: reg
                .find_iter(pts[1])
                .map(|instr| match instr.as_str() {
                    "L" => Instr::Turn(false),
                    "R" => Instr::Turn(true),
                    n => Instr::Step(n.parse().unwrap()),
                })
                .collect(),
        }
    }

    fn walk<F>(&self, step: F) -> i32
    where
        F: Fn(&[Vec<char>], C<i32>, C<i32>) -> (C<i32>, C<i32>),
    {
        let mut pos = iterate(self.top_left, |p| p + C(0, 1))
            .find(|&p| self.grid[p] == '.')
            .unwrap();
        let mut dir = C(0, 1);
        for instr in self.path.iter() {
            match instr {
                Instr::Turn(false) => dir *= C(0, 1),
                Instr::Turn(true) => dir *= C(0, -1),
                Instr::Step(n) => {
                    for _ in 0..*n {
                        let (pos2, dir2) = step(&self.grid, pos, dir);
                        if self.grid[pos2] == '#' {
                            break;
                        }
                        (pos, dir) = (pos2, dir2);
                    }
                }
            }
        }
        let C(row, col) = pos + C(1, 1);
        let facing = match dir {
            C(0, 1) => 0,
            C(1, 0) => 1,
            C(0, -1) => 2,
            C(-1, 0) => 3,
            _ => unreachable!(),
        };
        1000 * row + 4 * col + facing
    }

    fn cube_edges(&self) -> AHashMap<Pt, Pt> {
        let cube_size = gcd(
            self.grid.len(),
            self.grid.iter().map(|r| r.len()).max().unwrap(),
        ) as i32;
        let mut edges = AHashMap::new();
        let mut pos = self.top_left;
        let mut dir = C(-1, 0);
        let mut pos3d = C3(0, cube_size - 1, cube_size - 1);
        let mut dir3d = C3(0, 1, 0);
        let mut face = Top;
        let mut first = true;
        while std::mem::take(&mut first) || pos != self.top_left {
            let (i, (d2, d3)) = [
                (dir * C(0, 1), face.rotate_left(dir3d)),
                (dir, dir3d),
                (dir * C(0, -1), face.rotate_right(dir3d)),
            ]
            .into_iter()
            .enumerate()
            .find(|(_, (d2, _))| {
                (pos + d2)
                    .index_of(&self.grid)
                    .filter(|&p| self.grid[p] != ' ')
                    .is_some()
            })
            .unwrap();
            let e = edges.entry(pos3d).or_insert_with(Vec::new);
            if i > 0 {
                e.push(Edge {
                    pos,
                    dir: dir * C(0, 1),
                    src: face,
                    dest: face.fall_off(face.rotate_left(dir3d)).0,
                });
            }
            if i > 1 {
                e.push(Edge {
                    pos,
                    dir,
                    src: face,
                    dest: face.fall_off(dir3d).0,
                })
            }
            ((face, dir3d), pos3d) = if d2.0 == 1 && (pos.0 + 1) % cube_size == 0
                || d2.1 == 1 && (pos.1 + 1) % cube_size == 0
                || d2.0 == -1 && pos.0 % cube_size == 0
                || d2.1 == -1 && pos.1 % cube_size == 0
            {
                (face.fall_off(d3), pos3d)
            } else {
                ((face, d3), pos3d + d3)
            };
            dir = d2;
            pos += dir;
        }
        let mut edge_map = AHashMap::new();
        for pts in edges.values() {
            for (i, a) in pts.iter().enumerate() {
                for b in pts.iter().skip(i + 1) {
                    if a.src == b.dest && b.src == a.dest {
                        edge_map.insert((a.pos, a.dir), (b.pos, -b.dir));
                        edge_map.insert((b.pos, b.dir), (a.pos, -a.dir));
                    }
                }
            }
        }
        edge_map
    }
}

#[derive(Debug)]
struct Edge {
    pos: C<i32>,
    dir: C<i32>,
    src: Face,
    dest: Face,
}

pub fn part1(input: &str) -> i32 {
    Board::new(input).walk(|grid, C(r, c), C(dr, dc)| {
        let pos = if dr != 0 {
            let mr = grid.len() as i32;
            iterate(r, |r| (r + dr).rem_euclid(mr))
                .map(|r| C(r, c))
                .skip(1)
                .find(|&p| p.index_of(grid).filter(|&p| grid[p] != ' ').is_some())
                .unwrap()
        } else {
            let mc = grid[r as usize].len() as i32;
            iterate(c, |c| (c + dc).rem_euclid(mc))
                .map(|c| C(r, c))
                .skip(1)
                .find(|&p| p.index_of(grid).filter(|&p| grid[p] != ' ').is_some())
                .unwrap()
        };
        (pos, C(dr, dc))
    })
}

pub fn part2(input: &str) -> i32 {
    let board = Board::new(input);
    let edges = board.cube_edges();
    board.walk(|_, pos, dir| {
        (pos + dir)
            .index_of(&board.grid)
            .filter(|&p| board.grid[p] != ' ')
            .map(|p| (p, dir))
            .unwrap_or_else(|| edges[&(pos, dir)])
    })
}
