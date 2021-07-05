use cached::proc_macro::cached;
use cached::Cached;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Grid {
    arr: Vec<char>,
    cols: usize,
}

fn parse_maze(input: &str) -> Grid {
    let cols = input.lines().next().unwrap().len();
    Grid {
        arr: input.lines().flat_map(|line| line.chars()).collect(),
        cols: cols,
    }
}

fn conv(c: char) -> u32 {
    c as u32 - 'a' as u32
}

#[cached(
    name = "CACHE2",
    key = "(u32, usize)",
    convert = r#"{ (found, start) }"#
)]
fn dists_to_keys(grid: &Grid, found: u32, start: usize) -> Vec<(char, usize, usize)> {
    fn neighbors<'a>(
        grid: &'a Grid,
        found: u32,
        node: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let (pos, depth) = node;
        vec![pos - grid.cols, pos - 1, pos + 1, pos + grid.cols]
            .into_iter()
            .filter_map(move |p| {
                let v = grid.arr[p];
                (v != '#'
                    && (!v.is_ascii_uppercase() || found >> conv(v.to_ascii_lowercase()) & 1 != 0))
                    .then(|| (p, depth + 1))
            })
    }

    let mut visited: HashSet<usize> = vec![start].into_iter().collect();
    let mut frontier: VecDeque<(usize, usize)> = neighbors(grid, found, (start, 0)).collect();
    let mut result = Vec::new();
    while let Some((pos, depth)) = frontier.pop_front() {
        if !visited.insert(pos) {
            continue;
        }
        let k = grid.arr[pos];
        if k.is_ascii_lowercase() && found >> conv(k) & 1 == 0 {
            result.push((k, depth, pos));
        } else {
            frontier.extend(neighbors(grid, found, (pos, depth)));
        }
    }
    result
}

#[cached(
    name = "CACHE1",
    key = "(Vec<usize>, u32)",
    convert = r#"{ (starts.clone(), found) }"#
)]
fn go(grid: &Grid, starts: Vec<usize>, found: u32) -> usize {
    starts
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            dists_to_keys(grid, found, *p)
                .into_iter()
                .map(|(ch, dist, pos)| {
                    let mut starts2 = starts.clone();
                    starts2[i] = pos;
                    dist + go(grid, starts2, found | 1 << conv(ch))
                })
                .collect::<Vec<_>>()
        })
        .min()
        .unwrap_or(0)
}

fn search(grid: Grid, key: char) -> usize {
    let key_poss = grid
        .arr
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (v == &key).then(|| i))
        .collect();
    go(&grid, key_poss, 0)
}

pub fn part1(input: &str) -> usize {
    CACHE1.lock().unwrap().cache_clear();
    CACHE2.lock().unwrap().cache_clear();
    search(parse_maze(input), '@')
}

pub fn part2(input: &str) -> usize {
    CACHE1.lock().unwrap().cache_clear();
    CACHE2.lock().unwrap().cache_clear();
    let mut maze = parse_maze(input);
    for (k, v) in (39..=41)
        .flat_map(|x| (39..=41).map(move |y| (x, y)))
        .zip("@#@###@#@".chars())
    {
        maze.arr[k.0 * maze.cols + k.1] = v;
    }
    search(maze, '@')
}
