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
        let (c1, v1, _c2, v2a, v2b) =
            scan_fmt!(line, "{}={}, {}={}..{}", char, i32, char, i32, i32).unwrap();
        let (xs, ys) = if c1 == 'x' {
            (vec![v1], (v2a..=v2b).collect())
        } else {
            ((v2a..=v2b).collect(), vec![v1])
        };
        for x in xs {
            for y in &ys {
                clay.push((x, *y));
            }
        }
    }
    let (x0, y0) = (
        clay.iter().map(|v| v.0).min().unwrap() - 1,
        clay.iter().map(|v| v.1).min().unwrap(),
    );
    let (x1, y1) = (
        clay.iter().map(|v| v.0).max().unwrap() + 1,
        clay.iter().map(|v| v.1).max().unwrap(),
    );
    let grid = vec![vec!['.'; (y1 - y0 + 1) as usize]; (x1 - x0 + 1) as usize];
    let mut ground = Ground {
        grid: grid,
        offset_x: x0,
        min_y: y0,
        max_y: y1,
    };
    for (r, c) in clay {
        ground.set((r, c), '#');
    }
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
        if coord.1 > self.max_y {
            return false;
        }
        if self.get(coord) == '|' {
            return false;
        }
        if self.get(coord) == '#' {
            return true;
        }
        if !self.go((coord.0, coord.1 + 1)) {
            if self.get(coord) == '.' {
                self.set(coord, '|');
            }
            return false;
        } else {
            let lefts = self.spread(coord, left);
            let rights = self.spread(coord, right);
            let next_l = left(&lefts[lefts.len() - 1]);
            let next_r = right(&rights[rights.len() - 1]);
            if self.get(next_l) == '#' && self.get(next_r) == '#' {
                lefts
                    .into_iter()
                    .chain(rights)
                    .for_each(|c| self.set(c, '~'));
                true
            } else {
                lefts
                    .into_iter()
                    .chain(rights)
                    .for_each(|c| self.set(c, '|'));
                let (a, b) = (self.go(next_l), self.go(next_r));
                a && b
            }
        }
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
