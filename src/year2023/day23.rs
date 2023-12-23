use crate::utils::*;
use ahash::AHashMap;
use Tile::*;

const ADJ: [C<i32>; 4] = [C(0, -1), C(0, 1), C(-1, 0), C(1, 0)];

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Slope(C<i32>),
    Start,
    End,
    Fork,
}

struct Maze {
    p2: bool,
    grid: Grid<Tile, i32>,
    moves: AHashMap<C<i32>, Vec<(usize, C<i32>)>>,
}

fn open_adj(grid: &Grid<u8, i32>, p: C<i32>) -> usize {
    ADJ.into_iter()
        .filter(|d| matches!(grid.get(p + d), Some(v) if *v != b'#'))
        .count()
}

impl Maze {
    fn new(input: &str, p2: bool) -> (Self, C<i32>) {
        let grid: Grid<_, i32> = input.bytes().collect();
        let (start, end) = (C(0, 1), C(grid.rows - 1, grid.cols - 2));
        let grid = grid.clone().itransform(|(p, v)| match v {
            b'<' => Slope(C(0, -1)),
            b'>' => Slope(C(0, 1)),
            b'^' => Slope(C(-1, 0)),
            b'v' => Slope(C(1, 0)),
            b'#' => Wall,
            b'.' if p == start => Start,
            b'.' if p == end => End,
            b'.' if open_adj(&grid, p) > 2 => Fork,
            b'.' => Floor,
            _ => unreachable!(),
        });
        (
            Self {
                p2,
                grid,
                moves: AHashMap::new(),
            },
            start,
        )
    }

    fn available_moves(&mut self, pos: &C<i32>) -> Vec<(usize, C<i32>)> {
        if !self.moves.contains_key(pos) {
            let mut moves = Vec::new();
            moves.extend(
                bfs(*pos, |st| {
                    if st != pos && [Start, End, Fork].contains(&self.grid[*st]) {
                        return vec![];
                    }
                    ADJ.into_iter()
                        .filter_map(|d| {
                            let p = st + d;
                            self.grid.get(p).and_then(|&v| match v {
                                Slope(s) if !self.p2 && s != d => None,
                                Wall => None,
                                _ => Some(p),
                            })
                        })
                        .collect()
                })
                .filter(|&(_, st)| [Start, End, Fork].contains(&self.grid[st])),
            );
            self.moves.insert(*pos, moves);
        }
        self.moves[pos].clone()
    }

    fn dfs(&mut self, visited: &mut Grid<bool, i32>, pos: C<i32>) -> Option<usize> {
        if self.grid[pos] == End {
            return Some(0);
        }
        if visited[pos] {
            return None;
        }
        visited[pos] = true;
        let mut max = None;
        for (d, p) in self.available_moves(&pos) {
            if let Some(dist) = self.dfs(visited, p) {
                max = Some(max.unwrap_or(0).max(dist + d));
            }
        }
        visited[pos] = false;
        max
    }
}

pub fn part1(input: &str) -> Option<usize> {
    let (mut maze, start) = Maze::new(input, false);
    maze.dfs(&mut maze.grid.same_size(), start)
}

pub fn part2(input: &str) -> Option<usize> {
    let (mut maze, start) = Maze::new(input, true);
    maze.dfs(&mut maze.grid.same_size(), start)
}
