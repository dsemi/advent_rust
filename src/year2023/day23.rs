use crate::utils::*;
use rayon::prelude::*;
use Tile::*;

const ADJ: [C<i32>; 4] = [C(0, -1), C(0, 1), C(-1, 0), C(1, 0)];

#[derive(Copy, Clone)]
enum Tile {
    Wall,
    Floor,
    Slope(C<i32>),
    Start,
    End(usize),
    Fork,
}

struct Maze {
    end: usize,
    adj: Vec<Vec<(usize, usize)>>,
}

fn open_adj(grid: &Grid<u8, i32>, p: C<i32>) -> usize {
    ADJ.into_iter()
        .filter(|d| matches!(grid.get(p + d), Some(v) if *v != b'#'))
        .count()
}

fn neighbors(grid: &Grid<Tile, i32>, p2: bool, pos: &C<i32>) -> Vec<(usize, C<i32>)> {
    bfs(*pos, |st| {
        if st != pos && matches!(grid[*st], Start | End(_) | Fork) {
            return vec![];
        }
        ADJ.into_iter()
            .filter_map(|d| {
                let p = st + d;
                grid.get(p).and_then(|&v| match v {
                    Slope(s) if !p2 && s != d => None,
                    Wall => None,
                    _ => Some(p),
                })
            })
            .collect()
    })
    .skip(1)
    .filter(|&(_, st)| matches!(grid[st], Start | End(_) | Fork))
    .collect()
}

impl Maze {
    fn new(input: &str, p2: bool) -> Self {
        let grid: Grid<_, i32> = input.bytes().collect();
        let (start, mut end) = (C(0, 1), C(grid.rows - 1, grid.cols - 2));
        let mut grid = grid.clone().itransform(|(p, v)| match v {
            b'<' => Slope(C(0, -1)),
            b'>' => Slope(C(0, 1)),
            b'^' => Slope(C(-1, 0)),
            b'v' => Slope(C(1, 0)),
            b'#' => Wall,
            b'.' if p == start => Start,
            b'.' if p == end => End(0),
            b'.' if open_adj(&grid, p) > 2 => Fork,
            b'.' => Floor,
            _ => unreachable!(),
        });
        // Move end into last fork to avoid unnecessary pathing.
        let fork_before_end = neighbors(&grid, true, &end);
        assert_eq!(fork_before_end.len(), 1);
        grid[fork_before_end[0].1] = End(fork_before_end[0].0);
        grid[end] = Wall;
        end = fork_before_end[0].1;

        let mut ui = UniqueIdx::new();
        let mut adj: Vec<Vec<(usize, usize)>> = Vec::new();
        for (p, &v) in grid.idx_iter() {
            if matches!(v, Start | End(_) | Fork) {
                let i = ui.idx(p);
                if i >= adj.len() {
                    adj.resize(i + 1, vec![]);
                }
                adj[i] = neighbors(&grid, p2, &p)
                    .into_iter()
                    .map(|(j, pos)| {
                        let d = if let End(d) = grid[pos] { d } else { 0 };
                        (j + d, ui.idx(pos))
                    })
                    .collect();
            }
        }
        Self {
            end: ui.idx(end),
            adj,
        }
    }

    fn dfs(&self, vis: u64, pos: usize, dist: usize) -> Option<usize> {
        if pos == self.end {
            return Some(dist);
        }
        // Somewhat arbitrary number to limit amount of attempted parallelism
        if vis.count_ones() < 10 {
            self.adj[pos]
                .par_iter()
                .filter(|&(_, p)| vis & (1 << p) == 0)
                .filter_map(|&(d, p)| self.dfs(vis | (1 << p), p, dist + d))
                .max()
        } else {
            self.adj[pos]
                .iter()
                .filter(|&(_, p)| vis & (1 << p) == 0)
                .filter_map(|&(d, p)| self.dfs(vis | (1 << p), p, dist + d))
                .max()
        }
    }
}

fn solve(input: &str, p2: bool) -> usize {
    Maze::new(input, p2).dfs(1 | (1 << 1), 1, 91).unwrap()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
