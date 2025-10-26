use crate::utils::*;
use hashbrown::HashMap;

type Grid = Vec<Vec<bool>>;

fn parse_img(input: &str) -> Grid {
    input.split('/').map(|row| row.chars().map(|c| c == '#').collect()).collect()
}

fn parse_expansions(input: &str) -> HashMap<Grid, Grid> {
    input
        .lines()
        .flat_map(|line| {
            let (k, rest) = line.split_once(" => ").unwrap();
            let v = parse_img(rest);
            let mut grid = parse_img(k);
            (0..4)
                .flat_map(|_| {
                    grid = transpose(&grid);
                    let a = (grid.clone(), v.clone());
                    grid.reverse();
                    vec![a, (grid.clone(), v.clone())]
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn sqr(grid: &[Vec<bool>], i: usize, j: usize, span: usize) -> Grid {
    (i..i + span).map(|x| grid[x][j..j + span].to_vec()).collect()
}

fn expand_image(img: &[Vec<bool>], m: &HashMap<Grid, Grid>) -> Grid {
    let size = img.len();
    let span = if size.is_multiple_of(2) { 2 } else { 3 };
    let sq_size = size / span;
    let new_span = span + 1;
    let new_size = size * new_span / span;
    let mut result = vec![vec![false; new_size]; new_size];
    for r in 0..sq_size {
        for c in 0..sq_size {
            let sq = sqr(img, r * span, c * span, span);
            let exp = &m[&sq];
            for (x, row) in exp.iter().enumerate() {
                for (y, v) in row.iter().enumerate() {
                    result[r * new_span + x][c * new_span + y] = *v;
                }
            }
        }
    }
    result
}

fn count_px_after_expanding(input: &str, n: usize) -> usize {
    let m = parse_expansions(input);
    let mut img = parse_img(".#./..#/###");
    for _ in 0..n {
        img = expand_image(&img, &m);
    }
    img.into_iter().map(|row| row.into_iter().filter(|x| *x).count()).sum()
}

pub fn part1(input: &str) -> usize {
    count_px_after_expanding(input, 5)
}

pub fn part2(input: &str) -> usize {
    count_px_after_expanding(input, 18)
}
