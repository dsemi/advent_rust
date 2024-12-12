use crate::utils::*;
use hashbrown::HashMap;

const DIM: i32 = 6;

struct Graph {
    base_cost: u32,
    down_cost: Grid<u32, i32>,
    right_cost: Grid<u32, i32>,
}

impl Graph {
    fn pathfind(&mut self, grid: &Grid<u8, i32>, coord: C<i32>, dir: C<i32>, node: C<i32>) -> u32 {
        let mut step = dir;
        let mut coord = coord + dir;
        if coord.0 == grid.rows || grid[coord] == b'#' {
            return 0;
        }
        let (mut length, mut markers_seen) = (1, 0);
        while markers_seen < 2 {
            length += 1;
            if grid[coord] != b'.' {
                markers_seen += 1;
            }
            for next_step in [step, C(-step.1, step.0), C(step.1, -step.0)] {
                let next_coord = coord + next_step;
                if grid[next_coord] != b'#' {
                    coord = next_coord;
                    step = next_step;
                    break;
                }
            }
        }
        let node = node + dir.swol(step);
        if self.down_cost[node] == 0 {
            self.down_cost[node] = self.pathfind(grid, coord, C(1, 0), node);
        }
        if self.right_cost[node] == 0 {
            self.right_cost[node] = self.pathfind(grid, coord, C(0, 1), node);
        }
        length
    }

    fn parse(input: &str) -> Self {
        let mut grid: Grid<u8, i32> = input.bytes().collect();
        grid[C(1, 1)] = b'v';
        let (rows, cols) = (grid.rows, grid.cols);
        grid[C(rows - 2, cols - 2)] = b'v';
        let mut graph = Self {
            base_cost: 0,
            down_cost: Grid::new(DIM + 1, DIM + 1),
            right_cost: Grid::new(DIM + 1, DIM + 1),
        };
        graph.base_cost = graph.pathfind(&grid, C(0, 1), C(1, 0), C(-1, 0));
        graph
    }
}

pub fn part1(input: &str) -> u32 {
    let graph = Graph::parse(input);
    let mut from_above = vec![0; DIM as usize];
    for r in 0..DIM {
        let mut from_left = 0;
        for c in 0..DIM {
            let cost = from_left.max(from_above[c as usize]);
            from_above[c as usize] = cost + graph.down_cost[C(r, c)];
            from_left = cost + graph.right_cost[C(r, c)];
        }
    }
    graph.base_cost + from_above.last().unwrap()
}

fn matching_paren(key: &[u8], mut col: i32, step: i32) -> i32 {
    let mut nest = step;
    while nest != 0 {
        col += step;
        if key[col as usize] == b'(' {
            nest += 1;
        } else if key[col as usize] == b')' {
            nest -= 1;
        }
    }
    col
}

fn transition(
    graph: &Graph,
    dp: &mut HashMap<[u8; 7], u32>,
    mut key: [u8; 7],
    cost: u32,
    row: i32,
    col: i32,
    left: u8,
) {
    if col == DIM {
        if left == b'.' {
            let e = dp.entry(key).or_default();
            *e = (*e).max(cost);
        }
        return;
    }
    let mut next_col = |key: &mut [u8; 7], down, right| {
        let mut cost_delta = 0;
        if down != b'.' {
            cost_delta += graph.down_cost[C(row, col)];
        }
        if right != b'.' {
            cost_delta += graph.right_cost[C(row, col)];
        }
        key[col as usize] = down;
        transition(graph, dp, *key, cost + cost_delta, row, col + 1, right);
    };

    let up = key[col as usize];
    match (left, up) {
        (b'.', b'(') | (b'.', b')') | (b'(', b'.') | (b')', b'.') => {
            next_col(&mut key, up, left);
            next_col(&mut key, left, up);
        }
        (b'.', b'.') => {
            next_col(&mut key, b'(', b')');
            next_col(&mut key, b'.', b'.');
        }
        (b'(', b'(') => {
            key[matching_paren(&key, col, 1) as usize] = b'(';
            next_col(&mut key, b'.', b'.');
        }
        (b')', b')') => {
            key[matching_paren(&key, col, -1) as usize] = b')';
            next_col(&mut key, b'.', b'.');
        }
        (b')', b'(') => {
            next_col(&mut key, b'.', b'.');
        }
        _ => (),
    }
}

pub fn part2(input: &str) -> u32 {
    let graph = Graph::parse(input);
    let mut dp = HashMap::new();
    dp.insert([b'(', b'.', b'.', b'.', b'.', b'.', b')'], graph.base_cost);
    for row in 0..DIM {
        let mut tmp = HashMap::new();
        dp.into_iter()
            .for_each(|(key, cost)| transition(&graph, &mut tmp, key, cost, row, 0, b'.'));
        dp = tmp;
    }
    dp[b".....()"]
}
