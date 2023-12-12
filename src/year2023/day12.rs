use crate::utils::parsers::*;
use rayon::prelude::*;
use std::cmp::Ordering::*;

struct Record {
    pattern: Vec<u8>,
    springs: Vec<usize>,
}

impl Record {
    fn parse(line: &str, reps: usize) -> Self {
        let (pat, ss) = line.split_once(' ').unwrap();
        let ns = list(usize).read(ss);
        Record {
            pattern: vec![pat; reps].join("?").bytes().collect(),
            springs: vec![ns; reps].into_iter().flatten().collect(),
        }
    }

    fn count_arrangements(&self) -> usize {
        let mut cache = vec![vec![None; self.springs.len() + 1]; self.pattern.len()];
        self.count(&mut cache, 0, 0)
    }

    fn count(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if i == self.pattern.len() {
            return if j == self.springs.len() { 1 } else { 0 };
        }
        if let Some(v) = cache[i][j] {
            return v;
        }
        let res = match self.pattern[i] {
            b'.' => self.count(cache, i + 1, j),
            b'#' => self.count_broken(cache, i, j),
            b'?' => self.count(cache, i + 1, j) + self.count_broken(cache, i, j),
            _ => unreachable!(),
        };
        cache[i][j] = Some(res);
        res
    }

    fn count_broken(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if j == self.springs.len() {
            return 0;
        }
        let end_group_idx = i + self.springs[j];
        if !self.broken_group_possible(i, end_group_idx) {
            return 0;
        }
        if end_group_idx == self.pattern.len() {
            return if j == self.springs.len() - 1 { 1 } else { 0 };
        }
        self.count(cache, end_group_idx + 1, j + 1)
    }

    fn broken_group_possible(&self, from: usize, to: usize) -> bool {
        match to.cmp(&self.pattern.len()) {
            Greater => false,
            Equal => self.pattern[from..to].iter().all(|&b| b != b'.'),
            Less => self.pattern[from..to].iter().all(|&b| b != b'.') && self.pattern[to] != b'#',
        }
    }
}

fn solve(input: &str, reps: usize) -> usize {
    input
        .par_lines()
        .map(|line| Record::parse(line, reps).count_arrangements())
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 1)
}

pub fn part2(input: &str) -> usize {
    solve(input, 5)
}
