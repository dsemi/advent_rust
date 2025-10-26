use crate::utils::parsers::*;
use crate::utils::*;

fn is_winner(grid: &Grid<i32>) -> bool {
    (0..grid.rows).any(|r| grid.row(r).all(|&v| v == -1))
        || (0..grid.cols).any(|c| grid.col(c).all(|&v| v == -1))
}

fn winner_scores(input: &str) -> impl Iterator<Item = i32> + '_ {
    let (nums, boards) = input.split_once("\n\n").unwrap();
    let mut boards: Vec<_> = boards
        .split("\n\n")
        .map(|b| Grid { rows: 5, cols: 5, elems: b.split_whitespace().map(|v| v.i32()).collect() })
        .collect();
    nums.split(',').flat_map(move |n| {
        let n = n.i32();
        boards
            .extract_if(.., move |b| {
                b.iter_mut().filter(|v| **v == n).for_each(|v| *v = -1);
                is_winner(b)
            })
            .map(move |b| b.into_iter().filter(|&n| n != -1).sum::<i32>() * n)
            .collect::<Vec<_>>()
    })
}

pub fn part1(input: &str) -> Option<i32> {
    winner_scores(input).next()
}

pub fn part2(input: &str) -> Option<i32> {
    winner_scores(input).last()
}
