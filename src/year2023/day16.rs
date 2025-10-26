use crate::utils::*;
use rayon::prelude::*;

#[derive(Clone)]
struct Visited {
    vis_ns: Grid<bool, i8>,
    vis_ew: Grid<bool, i8>,
}

impl Visited {
    fn new(dim: i8) -> Self {
        Self { vis_ns: Grid::new(dim, dim), vis_ew: Grid::new(dim, dim) }
    }

    fn expand_ns(&mut self, grid: &Grid<u8, i8>, pos: C<i8>, mut dir: C<i8>) {
        if let Some(v) = grid.get(pos) {
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

    fn expand_ew(&mut self, grid: &Grid<u8, i8>, pos: C<i8>, mut dir: C<i8>) {
        if let Some(v) = grid.get(pos) {
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
        self.vis_ns.iter().zip(self.vis_ew.iter()).filter(|&(a, b)| *a || *b).count()
    }
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<_, i8> = input.bytes().collect();
    assert_eq!(grid.rows, grid.cols);
    let mut tiles = Visited::new(grid.rows);
    tiles.expand_ew(&grid, C(0, 0), C(0, 1));
    tiles.energized()
}

pub fn part2(input: &str) -> Option<usize> {
    let grid: Grid<_, i8> = input.bytes().collect();
    assert_eq!(grid.rows, grid.cols);
    let tiles = Visited::new(grid.rows);
    let dim = grid.rows;
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
