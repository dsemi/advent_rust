use crate::utils::*;
use ahash::AHashMap;
use itertools::Itertools;

type Adj = AHashMap<(char, char), usize>;

fn neighbors(grid: &[Vec<char>], xy: &Coord<i32>) -> Vec<Coord<i32>> {
    vec![
        Coord::new(1, 0),
        Coord::new(-1, 0),
        Coord::new(0, 1),
        Coord::new(0, -1),
    ]
    .iter()
    .filter_map(|d| {
        let c = xy + d;
        (c.x >= 0
            && c.x < grid.len() as i32
            && c.y >= 0
            && c.y < grid[c.x as usize].len() as i32
            && grid[c.x as usize][c.y as usize] != '#')
            .then(|| c)
    })
    .collect()
}

fn find_all_distances(grid: &[Vec<char>], ns: &[(Coord<i32>, char)]) -> Adj {
    let mut result = AHashMap::new();
    for (p1, n1) in ns {
        for (p2, n2) in ns {
            if p1 == p2 {
                result.insert((*n1, *n2), 0);
                continue;
            }
            for (d, n) in bfs(*p1, |x| neighbors(grid, x)) {
                if n == *p2 {
                    result.insert((*n1, *n2), d);
                    break;
                }
            }
        }
    }
    result
}

fn all_paths_and_dist_map(input: &str) -> (Adj, Vec<Vec<char>>) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    let mut pts = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().filter_map(move |(c, v)| {
                v.is_ascii_digit()
                    .then(|| (Coord::new(r as i32, c as i32), v))
            })
        })
        .collect::<Vec<_>>();
    pts.sort_by_key(|x| x.1);
    let start = pts[0].1;
    let all_paths = pts[1..]
        .iter()
        .map(|x| x.1)
        .permutations(pts.len() - 1)
        .map(|perm| {
            let mut v = vec![start];
            v.extend(perm);
            v
        })
        .collect();
    let dist_map = find_all_distances(&grid, &pts);
    (dist_map, all_paths)
}

fn min_path_len(dists: Adj, all_paths: Vec<Vec<char>>) -> Option<usize> {
    all_paths
        .into_iter()
        .map(|xs| xs.iter().zip(&xs[1..]).map(|(&a, &b)| dists[&(a, b)]).sum())
        .min()
}

pub fn part1(input: &str) -> Option<usize> {
    let (dists, all_paths) = all_paths_and_dist_map(input);
    min_path_len(dists, all_paths)
}

pub fn part2(input: &str) -> Option<usize> {
    let (dists, mut all_paths) = all_paths_and_dist_map(input);
    for path in all_paths.iter_mut() {
        path.push('0');
    }
    min_path_len(dists, all_paths)
}
