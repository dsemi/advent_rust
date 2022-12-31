struct Valley {
    w: usize,
    h: usize,
    n_blizz: Vec<u64>,
    s_blizz: Vec<u64>,
    e_blizz: Vec<u64>,
    w_blizz: Vec<u64>,
    walls: Vec<u64>,
}

impl Valley {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let h = grid.len();
        let w = grid[0].len();
        let mut n_blizz = vec![0; w];
        let mut s_blizz = vec![0; w];
        let mut w_blizz = vec![0; w];
        let mut e_blizz = vec![0; w];
        let mut walls = vec![0; w];
        for (r, row) in grid.into_iter().enumerate() {
            for (c, v) in row.into_iter().enumerate() {
                match v {
                    '^' => n_blizz[c] |= 1 << r,
                    'v' => s_blizz[c] |= 1 << r,
                    '<' => w_blizz[c] |= 1 << r,
                    '>' => e_blizz[c] |= 1 << r,
                    '#' => walls[c] |= 1 << r,
                    _ => (),
                }
            }
        }
        Valley {
            w,
            h,
            n_blizz,
            s_blizz,
            w_blizz,
            e_blizz,
            walls,
        }
    }

    fn shortest_path(&mut self, start: (usize, usize), goal: (usize, usize)) -> usize {
        let mut t = 0;
        let mut frontier = vec![0; self.w];
        frontier[start.1] |= 1 << start.0;
        while frontier[goal.1] & 1 << goal.0 == 0 {
            t += 1;
            let pw_blizz = self.w_blizz.clone();
            let pe_blizz = self.e_blizz.clone();
            let p_frontier = frontier.clone();
            for c in 0..self.w {
                self.n_blizz[c] =
                    (self.n_blizz[c] >> 1 | (self.n_blizz[c] & 2) << (self.h - 3)) & !self.walls[c];
                self.s_blizz[c] =
                    (self.s_blizz[c] << 1 | (self.s_blizz[c] >> (self.h - 3) & 2)) & !self.walls[c];
                self.w_blizz[c] = pw_blizz[(c - 2 + self.w) % (self.w - 2) + 1];
                self.e_blizz[c] = pe_blizz[(c - 4 + self.w) % (self.w - 2) + 1];
                frontier[c] |= p_frontier[c] >> 1
                    | p_frontier[c] << 1
                    | p_frontier[(c + 1 + self.w) % self.w]
                    | p_frontier[(c - 1 + self.w) % self.w];
                frontier[c] &= !(self.walls[c]
                    | self.n_blizz[c]
                    | self.s_blizz[c]
                    | self.w_blizz[c]
                    | self.e_blizz[c]);
            }
        }
        t
    }
}

pub fn part1(input: &str) -> usize {
    let mut valley = Valley::new(input);
    valley.shortest_path((0, 1), (valley.h - 1, valley.w - 2))
}

pub fn part2(input: &str) -> usize {
    let mut valley = Valley::new(input);
    let (start, goal) = ((0, 1), (valley.h - 1, valley.w - 2));
    valley.shortest_path(start, goal)
        + valley.shortest_path(goal, start)
        + valley.shortest_path(start, goal)
}
