use crate::utils::*;

const UP: u8 = 0b1000;
const DOWN: u8 = 0b0100;
const LEFT: u8 = 0b0010;
const RIGHT: u8 = 0b0001;

fn coord(dir: u8) -> C<i32> {
    match dir {
        UP => C(-1, 0),
        DOWN => C(1, 0),
        LEFT => C(0, -1),
        RIGHT => C(0, 1),
        _ => unreachable!(),
    }
}

fn left(dir: u8) -> u8 {
    match dir {
        UP => LEFT,
        DOWN => RIGHT,
        LEFT => DOWN,
        RIGHT => UP,
        _ => unreachable!(),
    }
}

fn right(dir: u8) -> u8 {
    match dir {
        UP => RIGHT,
        DOWN => LEFT,
        LEFT => UP,
        RIGHT => DOWN,
        _ => unreachable!(),
    }
}

fn invert(dir: u8) -> u8 {
    match dir {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        _ => unreachable!(),
    }
}

fn dirs(v: char) -> u8 {
    match v {
        '|' => UP | DOWN,
        '-' => LEFT | RIGHT,
        'L' => UP | RIGHT,
        'J' => UP | LEFT,
        '7' => DOWN | LEFT,
        'F' => DOWN | RIGHT,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (C<i32>, u8, Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut start = None;
    for (r, row) in input.lines().enumerate() {
        let mut vs = Vec::new();
        for (c, v) in row.chars().enumerate() {
            vs.push(v);
            if v == 'S' {
                start = Some(C(r as i32, c as i32));
            }
        }
        grid.push(vs);
    }
    let start = start.unwrap();
    let dir = if matches!(grid.get_cell(start + coord(UP)), Some('|' | '7' | 'F')) {
        UP
    } else if matches!(grid.get_cell(start + coord(DOWN)), Some('|' | 'L' | 'J')) {
        DOWN
    } else if matches!(grid.get_cell(start + coord(LEFT)), Some('-' | 'L' | 'F')) {
        LEFT
    } else {
        RIGHT
    };
    (start, dir, grid)
}

pub fn part1(input: &str) -> usize {
    let (mut pos, mut dir, grid) = parse(input);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[pos] = true;
    pos += coord(dir);
    let mut cnt = 1;
    while !visited[pos] {
        cnt += 1;
        visited[pos] = true;
        dir = invert(dir) ^ dirs(grid[pos]);
        pos += coord(dir);
    }
    cnt / 2
}

fn update(lefts: &mut Vec<Vec<bool>>, rights: &mut Vec<Vec<bool>>, pos: C<i32>, dir: u8) {
    if let Some(v) = lefts.get_cell_mut(pos + coord(left(dir))) {
        *v = true;
    }
    if let Some(v) = rights.get_cell_mut(pos + coord(right(dir))) {
        *v = true;
    }
}

fn coords(grid: &[Vec<bool>]) -> Vec<C<i32>> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, v)| v.then_some(C(r as i32, c as i32)))
        })
        .collect()
}

fn dfs(main: &[Vec<bool>], vis: &mut [Vec<bool>], pos: C<i32>) -> usize {
    let mut res = 1;
    for d in [C(-1, 0), C(1, 0), C(0, -1), C(0, 1)] {
        let p = pos + d;
        if matches!(main.get_cell(p), Some(false)) && !vis[p] {
            vis[p] = true;
            res += dfs(main, vis, p);
        }
    }
    res
}

pub fn part2(input: &str) -> usize {
    let (mut pos, mut dir, grid) = parse(input);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut lefts = visited.clone();
    let mut rights = visited.clone();
    visited[pos] = true;
    update(&mut lefts, &mut rights, pos, dir);
    pos += coord(dir);
    while !visited[pos] {
        visited[pos] = true;
        update(&mut lefts, &mut rights, pos, dir);
        dir = invert(dir) ^ dirs(grid[pos]);
        update(&mut lefts, &mut rights, pos, dir);
        pos += coord(dir);
    }
    for (r, row) in visited.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            if *v {
                lefts[r][c] = false;
                rights[r][c] = false;
            }
        }
    }
    let left_starts = coords(&lefts);
    let left_cnt: usize = left_starts
        .into_iter()
        .map(|start| dfs(&visited, &mut lefts, start))
        .sum();
    let right_starts = coords(&rights);
    let right_cnt: usize = right_starts
        .into_iter()
        .map(|start| dfs(&visited, &mut rights, start))
        .sum();
    let c = (0..).find(|&i| !visited[0][i]).unwrap();
    if lefts[0][c] {
        right_cnt
    } else {
        left_cnt
    }
}
