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
    fn new(rows: usize, cols: usize) -> Self {
        let vis_ns = vec![vec![false; cols]; rows];
        let vis_ew = vec![vec![false; cols]; rows];
        Self { vis_ns, vis_ew }
    }

    fn expand_ns(&mut self, grid: &[Vec<u8>], pos: C<i32>, mut dir: C<i32>) {
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
                    self.expand_ew(grid, pos + C(0, -1), C(0, -1));
                    self.expand_ew(grid, pos + C(0, 1), C(0, 1));
                }
                _ => unreachable!(),
            }
        }
    }

    fn expand_ew(&mut self, grid: &[Vec<u8>], pos: C<i32>, mut dir: C<i32>) {
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
                    self.expand_ns(grid, pos + C(-1, 0), C(-1, 0));
                    self.expand_ns(grid, pos + C(1, 0), C(1, 0));
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
    let mut tiles = Visited::new(grid.len(), grid[0].len());
    tiles.expand_ew(&grid, C(0, 0), C(0, 1));
    tiles.energized()
}

pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let (rows, cols) = (grid.len(), grid[0].len());
    let tiles = Visited::new(rows, cols);
    let max_ew = (0..rows)
        .into_par_iter()
        .flat_map(|row| {
            [
                (C(row as i32, 0), C(0, 1)),
                (C(row as i32, cols as i32 - 1), C(0, -1)),
            ]
        })
        .map(|(pos, dir)| {
            let mut tiles = tiles.clone();
            tiles.expand_ew(&grid, pos, dir);
            tiles.energized()
        })
        .max()
        .unwrap();
    let max_ns = (0..cols)
        .into_par_iter()
        .flat_map(|col| {
            [
                (C(0, col as i32), C(1, 0)),
                (C(rows as i32 - 1, col as i32), C(-1, 0)),
            ]
        })
        .map(|(pos, dir)| {
            let mut tiles = tiles.clone();
            tiles.expand_ns(&grid, pos, dir);
            tiles.energized()
        })
        .max()
        .unwrap();
    max_ew.max(max_ns)
}
