use crate::utils::*;
use rayon::prelude::*;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[derive(Clone)]
struct Visited {
    vis_ns: Vec<Vec<bool>>,
    vis_ew: Vec<Vec<bool>>,
}

impl Visited {
    fn new(dim: usize) -> Self {
        let vis_ns = vec![vec![false; dim]; dim];
        let vis_ew = vec![vec![false; dim]; dim];
        Self { vis_ns, vis_ew }
    }

    fn expand_ns(&mut self, grid: &[Vec<u8>], pos: C<i8>, mut dir: C<i8>) {
        if let Some(v) = grid.get_cell(pos) {
            if std::mem::replace(&mut self.vis_ns[pos], true) && *v != b'/' && *v != b'\\' {
                return;
            }
            match v {
                b'.' | b'|' => self.expand_ns(grid, pos + dir, dir),
                b'/' => {
                    dir *= C(0, -1);
                    self.expand_ew(grid, pos + dir, dir);
                }
                b'\\' => {
                    dir *= C(0, 1);
                    self.expand_ew(grid, pos + dir, dir);
                }
                b'-' => {
                    dir *= C(0, -1);
                    self.expand_ew(grid, pos + dir, dir);
                    dir = -dir;
                    self.expand_ew(grid, pos + dir, dir);
                }
                _ => unreachable!(),
            }
        }
    }

    fn expand_ew(&mut self, grid: &[Vec<u8>], pos: C<i8>, mut dir: C<i8>) {
        if let Some(v) = grid.get_cell(pos) {
            if std::mem::replace(&mut self.vis_ew[pos], true) && *v != b'/' && *v != b'\\' {
                return;
            }
            match v {
                b'.' | b'-' => self.expand_ew(grid, pos + dir, dir),
                b'/' => {
                    dir *= C(0, 1);
                    self.expand_ns(grid, pos + dir, dir);
                }
                b'\\' => {
                    dir *= C(0, -1);
                    self.expand_ns(grid, pos + dir, dir);
                }
                b'|' => {
                    dir *= C(0, -1);
                    self.expand_ns(grid, pos + dir, dir);
                    dir = -dir;
                    self.expand_ns(grid, pos + dir, dir);
                }
                _ => unreachable!(),
            }
        }
    }

    fn energized(&self) -> usize {
        self.vis_ns
            .iter()
            .zip(self.vis_ew.iter())
            .flat_map(|(ns, ew)| ns.iter().zip(ew.iter()).filter(|&(a, b)| *a || *b))
            .count()
    }
}

pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    assert!(grid.len() == grid[0].len());
    let mut tiles = Visited::new(grid.len());
    tiles.expand_ew(&grid, C(0, 0), C(0, 1));
    tiles.energized()
}

pub fn part2(input: &str) -> Option<usize> {
    let grid = parse(input);
    assert!(grid.len() == grid[0].len());
    let tiles = Visited::new(grid.len());
    let dim = grid.len() as i8;
    (0..dim)
        .into_par_iter()
        .flat_map(|d| {
            [
                (C(d, 0), C(0, 1)),
                (C(d, dim - 1), C(0, -1)),
                (C(0, d), C(1, 0)),
                (C(dim - 1, d), C(-1, 0)),
            ]
        })
        .map(|(pos, dir)| {
            let mut tiles = tiles.clone();
            if dir.0 == 0 {
                tiles.expand_ew(&grid, pos, dir);
            } else {
                tiles.expand_ns(&grid, pos, dir);
            }
            tiles.energized()
        })
        .max()
}
