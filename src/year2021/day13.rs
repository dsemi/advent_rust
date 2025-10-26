use crate::utils::ocr::*;
use crate::utils::parsers::*;
use Fold::*;
use hashbrown::HashSet;
use std::cmp::{max, min};

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_fold(i: &mut &str) -> Result<Fold> {
    alt((preceded("fold along x=", usize).map(X), preceded("fold along y=", usize).map(Y)))
        .parse_next(i)
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, &str) {
    let (dots, instrs) = input.split_once("\n\n").unwrap();
    (lines_iter(dots, coord(usize)).collect(), instrs)
}

fn fold(paper: HashSet<(usize, usize)>, instr: &str) -> HashSet<(usize, usize)> {
    match parse_fold.read(instr) {
        X(n) => paper.into_iter().map(|(x, y)| (min(x, 2 * n - x), y)).collect(),
        Y(n) => paper.into_iter().map(|(x, y)| (x, min(y, 2 * n - y))).collect(),
    }
}

pub fn part1(input: &str) -> usize {
    let (paper, instrs) = parse(input);
    fold(paper, instrs.lines().next().unwrap()).len()
}

pub fn part2(input: &str) -> String {
    let (mut paper, instrs) = parse(input);
    for instr in instrs.lines() {
        paper = fold(paper, instr);
    }
    let (mx, my) = paper.iter().fold((0, 0), |(mx, my), (x, y)| (max(mx, *x), max(my, *y)));
    let mut display = vec!["".to_owned()];
    display.extend(
        (0..=my)
            .map(|y| (0..=mx).map(|x| if paper.contains(&(x, y)) { '#' } else { ' ' }).collect()),
    );
    parse_letters(&display.join("\n"), None)
}
