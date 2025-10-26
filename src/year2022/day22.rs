use crate::utils::parsers::*;
use crate::utils::*;
use Face::*;
use hashbrown::HashMap;
use itertools::iterate;
use num::integer::gcd;

const X: C3<i32> = C3(1, 0, 0);
const Y: C3<i32> = C3(0, 1, 0);
const Z: C3<i32> = C3(0, 0, 1);

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
    fn axis(&self) -> C3<i32> {
        match self {
            Top => Z,
            Bottom => -Z,
            Back => Y,
            Front => -Y,
            Right => X,
            Left => -X,
        }
    }

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
            -self.axis(),
        )
    }

    fn rotate(&self, C3(x, y, z): C3<i32>) -> C3<i32> {
        // Could use quaternion: (cos angle/2, x*sin angle/2, y*sin angle/2, z*sin angle/2)
        // e.g. rotate around Z: (cos pi / 4, 0, 0, sin pi / 4)
        // def mulQ(r, s):
        //   return (r[0]*s[0] - r[1]*s[1] - r[2]*s[2] - r[3]*s[3],
        //           r[0]*s[1] + r[1]*s[0] - r[2]*s[3] + r[3]*s[2],
        //           r[0]*s[2] + r[1]*s[3] + r[2]*s[0] - r[3]*s[1],
        //           r[0]*s[3] - r[1]*s[2] + r[2]*s[1] + r[3]*s[0])
        match self {
            Top => C3(-y, x, z),
            Bottom => C3(y, -x, z),
            Back => C3(z, y, -x),
            Front => C3(-z, y, x),
            Right => C3(x, -z, y),
            Left => C3(x, z, -y),
        }
    }
}

#[derive(Clone)]
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

fn instr(i: &mut &str) -> ModalResult<Instr> {
    alt(('L'.value(Instr::Turn(false)), 'R'.value(Instr::Turn(true)), usize.map(Instr::Step)))
        .parse_next(i)
}

impl Board {
    fn new(input: &str) -> Self {
        let (grid, path): (Vec<Vec<_>>, Vec<_>) =
            separated_pair(lines(repeat(1.., none_of('\n'))), "\n\n", repeat(1.., instr))
                .read(input);
        Self { top_left: C(0, grid[0].iter().position(|&c| c != ' ').unwrap() as i32), grid, path }
    }

    fn valid(&self, idx: C<i32>) -> bool {
        idx.0 >= 0
            && idx.0 < self.grid.len() as i32
            && idx.1 >= 0
            && idx.1 < self.grid[idx.0 as usize].len() as i32
            && self.grid[idx.0 as usize][idx.1 as usize] != ' '
    }

    fn walk<F>(&self, step: F) -> i32
    where
        F: Fn(&[Vec<char>], C<i32>, C<i32>) -> (C<i32>, C<i32>),
    {
        let mut pos = iterate(self.top_left, |p| p + C(0, 1))
            .find(|&C(r, c)| self.grid[r as usize][c as usize] == '.')
            .unwrap();
        let mut dir = C(0, 1);
        for instr in self.path.iter() {
            match instr {
                Instr::Turn(false) => dir *= C(0, 1),
                Instr::Turn(true) => dir *= C(0, -1),
                Instr::Step(n) => {
                    for _ in 0..*n {
                        let (pos2, dir2) = step(&self.grid, pos, dir);
                        if self.grid[pos2.0 as usize][pos2.1 as usize] == '#' {
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

    fn cube_edges(&self) -> HashMap<Pt, Pt> {
        let cube_size =
            gcd(self.grid.len(), self.grid.iter().map(|r| r.len()).max().unwrap()) as i32;
        let mut edges = HashMap::new();
        let mut pos = self.top_left;
        let mut dir = C(-1, 0);
        let mut pos3d = C3(0, cube_size - 1, cube_size - 1);
        let mut dir3d = C3(0, 1, 0);
        let mut face = Top;
        let mut first = true;
        while std::mem::take(&mut first) || pos != self.top_left {
            let (d2, d3) = [
                (dir * C(0, 1), face.rotate(dir3d)),
                (dir, dir3d),
                (dir * C(0, -1), face.rotate(-dir3d)),
            ]
            .into_iter()
            .find(|(d2, d3)| {
                if self.valid(pos + d2) {
                    return true;
                }
                let e = edges.entry(pos3d).or_insert_with(Vec::new);
                e.push(Edge { pos, dir: *d2, src: face, dest: face.fall_off(*d3).0 });
                false
            })
            .unwrap();
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
        let mut edge_map = HashMap::new();
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
    let board = Board::new(input);
    board.walk(|grid, C(r, c), C(dr, dc)| {
        let pos = if dr != 0 {
            let mr = grid.len() as i32;
            iterate(r, |r| (r + dr).rem_euclid(mr))
                .map(|r| C(r, c))
                .skip(1)
                .find(|&p| board.valid(p))
                .unwrap()
        } else {
            let mc = grid[r as usize].len() as i32;
            iterate(c, |c| (c + dc).rem_euclid(mc))
                .map(|c| C(r, c))
                .skip(1)
                .find(|&p| board.valid(p))
                .unwrap()
        };
        (pos, C(dr, dc))
    })
}

pub fn part2(input: &str) -> i32 {
    let board = Board::new(input);
    let edges = board.cube_edges();
    board.walk(|_, pos, dir| {
        let p = pos + dir;
        if board.valid(p) { (p, dir) } else { edges[&(pos, dir)] }
    })
}
