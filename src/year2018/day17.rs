use crate::utils::parsers::*;
use itertools::iterate;

struct Ground {
    grid: Vec<Vec<char>>,
    offset_x: i32,
    min_y: i32,
    max_y: i32,
}

fn parse_scans(input: &str) -> Ground {
    let mut clay = Vec::new();
    for line in input.lines() {
        let (c1, _, v1, _, v2a, _, v2b) =
            (any, '=', i32, (", ", any, '='), i32, "..", i32).read(line);
        if c1 == 'x' {
            (v2a..=v2b).for_each(|y| clay.push((v1, y)));
        } else {
            (v2a..=v2b).for_each(|x| clay.push((x, v1)));
        }
    }
    let (mut x0, mut y0) = (i32::MAX, i32::MAX);
    let (mut x1, mut y1) = (0, 0);
    for &(x, y) in &clay {
        x0 = std::cmp::min(x0, x - 1);
        x1 = std::cmp::max(x1, x + 1);
        y0 = std::cmp::min(y0, y);
        y1 = std::cmp::max(y1, y);
    }
    let grid = vec![vec!['.'; (y1 - y0 + 1) as usize]; (x1 - x0 + 1) as usize];
    let mut ground = Ground {
        grid,
        offset_x: x0,
        min_y: y0,
        max_y: y1,
    };
    clay.into_iter().for_each(|pos| ground.set(pos, '#'));
    ground
}

fn left(c: &(i32, i32)) -> (i32, i32) {
    (c.0 - 1, c.1)
}

fn right(c: &(i32, i32)) -> (i32, i32) {
    (c.0 + 1, c.1)
}

impl Ground {
    fn get(&self, c: (i32, i32)) -> char {
        self.grid[(c.0 - self.offset_x) as usize][(c.1 - self.min_y) as usize]
    }

    fn set(&mut self, c: (i32, i32), v: char) {
        self.grid[(c.0 - self.offset_x) as usize][(c.1 - self.min_y) as usize] = v;
    }

    fn spread(&self, c: (i32, i32), f: fn(&(i32, i32)) -> (i32, i32)) -> Vec<(i32, i32)> {
        iterate(c, f)
            .take_while(|&(x, y)| self.get((x, y)) != '#' && "#~".contains(self.get((x, y + 1))))
            .collect()
    }

    fn go(&mut self, coord: (i32, i32)) -> bool {
        if coord.1 < self.min_y {
            return self.go((coord.0, coord.1 + 1));
        }
        if coord.1 > self.max_y || self.get(coord) == '|' {
            return false;
        }
        if self.get(coord) == '#' {
            return true;
        }
        if !self.go((coord.0, coord.1 + 1)) {
            self.set(coord, '|');
            return false;
        }
        let lefts = self.spread(coord, left);
        let rights = self.spread(coord, right);
        let next_l = left(&lefts[lefts.len() - 1]);
        let next_r = right(&rights[rights.len() - 1]);
        let cond = self.get(next_l) == '#' && self.get(next_r) == '#';
        let v = if cond { '~' } else { '|' };
        lefts.into_iter().chain(rights).for_each(|c| self.set(c, v));
        if cond {
            return true;
        }
        // Don't want short-circuiting here.
        let (a, b) = (self.go(next_l), self.go(next_r));
        a && b
    }
}

fn flood(g: &mut Ground) {
    g.go((500, 0));
}

pub fn part1(input: &str) -> usize {
    let mut ground = parse_scans(input);
    flood(&mut ground);
    ground
        .grid
        .into_iter()
        .map(|col| col.into_iter().filter(|&c| "~|".contains(c)).count())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut ground = parse_scans(input);
    flood(&mut ground);
    ground
        .grid
        .into_iter()
        .map(|col| col.into_iter().filter(|&c| "~".contains(c)).count())
        .sum()
}
