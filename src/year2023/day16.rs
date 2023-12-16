use crate::utils::*;

#[derive(Clone)]
struct Tiles {
    grid: Vec<Vec<u8>>,
    vis_ns: Vec<Vec<bool>>,
    vis_ew: Vec<Vec<bool>>,
}

impl Tiles {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
        let vis_ns = vec![vec![false; grid[0].len()]; grid.len()];
        let vis_ew = vec![vec![false; grid[0].len()]; grid.len()];
        Tiles {
            grid,
            vis_ns,
            vis_ew,
        }
    }

    fn expand_ns(&mut self, pos: C<usize>, south: bool) {
        if let Some(v) = self.grid.get_cell(pos) {
            if std::mem::replace(&mut self.vis_ns[pos], true) && *v != b'/' && *v != b'\\' {
                return;
            }
            match v {
                b'.' | b'|' => {
                    self.expand_ns(if south { pos + C(1, 0) } else { pos - C(1, 0) }, south)
                }
                b'/' => {
                    if south {
                        self.expand_ew(pos - C(0, 1), false);
                    } else {
                        self.expand_ew(pos + C(0, 1), true);
                    }
                }
                b'\\' => {
                    if south {
                        self.expand_ew(pos + C(0, 1), true);
                    } else {
                        self.expand_ew(pos - C(0, 1), false);
                    }
                }
                b'-' => {
                    self.expand_ew(pos - C(0, 1), false);
                    self.expand_ew(pos + C(0, 1), true);
                }
                _ => unreachable!(),
            }
        }
    }

    fn expand_ew(&mut self, pos: C<usize>, east: bool) {
        if let Some(v) = self.grid.get_cell(pos) {
            if std::mem::replace(&mut self.vis_ew[pos], true) && *v != b'/' && *v != b'\\' {
                return;
            }
            match v {
                b'.' | b'-' => {
                    self.expand_ew(if east { pos + C(0, 1) } else { pos - C(0, 1) }, east)
                }
                b'/' => {
                    if east {
                        self.expand_ns(pos - C(1, 0), false);
                    } else {
                        self.expand_ns(pos + C(1, 0), true);
                    }
                }
                b'\\' => {
                    if east {
                        self.expand_ns(pos + C(1, 0), true);
                    } else {
                        self.expand_ns(pos - C(1, 0), false);
                    }
                }
                b'|' => {
                    self.expand_ns(pos - C(1, 0), false);
                    self.expand_ns(pos + C(1, 0), true);
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
    let mut tiles = Tiles::parse(input);
    tiles.expand_ew(C(0, 0), true);
    tiles.energized()
}

pub fn part2(input: &str) -> usize {
    let tiles = Tiles::parse(input);
    let mut max = usize::MIN;
    for row in 0..tiles.grid.len() {
        let mut tiles2 = tiles.clone();
        tiles2.expand_ew(C(row, 0), true);
        max = max.max(tiles2.energized());
        let mut tiles2 = tiles.clone();
        tiles2.expand_ew(C(row, tiles.grid[0].len() - 1), false);
        max = max.max(tiles2.energized());
    }
    for col in 0..tiles.grid[0].len() {
        let mut tiles2 = tiles.clone();
        tiles2.expand_ns(C(0, col), true);
        max = max.max(tiles2.energized());
        let mut tiles2 = tiles.clone();
        tiles2.expand_ns(C(tiles.grid.len() - 1, col), false);
        max = max.max(tiles2.energized());
    }
    max
}
