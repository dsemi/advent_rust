use crate::utils::*;
use rayon::prelude::*;
use smallvec::{smallvec, SmallVec};
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
    adj: Vec<SmallVec<[(usize, usize); 4]>>,
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
        let mut adj: Vec<SmallVec<[(usize, usize); 4]>> = Vec::new();
        for (p, &v) in grid.idx_iter() {
            if matches!(v, Start | End(_) | Fork) {
                let i = ui.idx(p);
                if i >= adj.len() {
                    adj.resize(i + 1, smallvec![]);
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

    fn dfs(&self, vis: u64, pos: usize, dist: usize) -> usize {
        if pos == self.end {
            return dist;
        }
        self.adj[pos]
            .iter()
            .filter(|&(_, p)| vis & (1 << p) == 0)
            .map(|&(d, p)| self.dfs(vis | (1 << p), p, dist + d))
            .max()
            .unwrap_or(0)
    }

    fn dfs_iter(&self) -> usize {
        let mut paths = Vec::new();
        let (dist, pos) = self.adj[0][0];
        let mut stack = vec![(1 | (1 << pos), pos, dist, 0)];
        while let Some((vis, pos, dist, depth)) = stack.pop() {
            // Somewhat arbitrary number to limit amount of attempted parallelism
            if depth >= 10 || pos == self.end {
                paths.push((vis, pos, dist));
                continue;
            }
            self.adj[pos]
                .iter()
                .filter(|&(_, p)| vis & (1 << p) == 0)
                .for_each(|&(d, p)| stack.push((vis | (1 << p), p, dist + d, depth + 1)))
        }
        paths
            .into_par_iter()
            .map(|(vis, pos, dist)| self.dfs(vis, pos, dist))
            .max()
            .unwrap()
    }
}

pub fn part1(input: &str) -> usize {
    Maze::new(input, false).dfs_iter()
}

pub fn part2(input: &str) -> usize {
    Maze::new(input, true).dfs_iter()
}