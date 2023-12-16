use crate::utils::*;

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
                    self.expand_ns(&grid, pos + dir, dir);
                }
                b'\\' => {
                    dir *= C(0, -1);
                    self.expand_ns(&grid, pos + dir, dir);
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
    let mut max = usize::MIN;
    for row in 0..rows {
        let mut tiles2 = tiles.clone();
        tiles2.expand_ew(&grid, C(row as i32, 0), C(0, 1));
        max = max.max(tiles2.energized());
        let mut tiles2 = tiles.clone();
        tiles2.expand_ew(&grid, C(row as i32, cols as i32 - 1), C(0, -1));
        max = max.max(tiles2.energized());
    }
    for col in 0..cols {
        let mut tiles2 = tiles.clone();
        tiles2.expand_ns(&grid, C(0, col as i32), C(1, 0));
        max = max.max(tiles2.energized());
        let mut tiles2 = tiles.clone();
        tiles2.expand_ns(&grid, C(rows as i32 - 1, col as i32), C(-1, 0));
        max = max.max(tiles2.energized());
    }
    max
}
