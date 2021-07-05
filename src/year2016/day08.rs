use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

const W: usize = 50;
const H: usize = 6;

fn process_instr(grid: &mut HashSet<(usize, usize)>, line: &str) {
    lazy_static! {
        static ref RE_RECT: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        static ref RE_ROW: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        static ref RE_COL: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    }
    if let Some(cap) = RE_RECT.captures(line) {
        let a: usize = cap[1].parse().unwrap();
        let b: usize = cap[2].parse().unwrap();
        for c in 0..a {
            for r in 0..b {
                grid.insert((r, c));
            }
        }
    } else if let Some(cap) = RE_ROW.captures(line) {
        let a: usize = cap[1].parse().unwrap();
        let b: usize = cap[2].parse().unwrap();
        *grid = grid
            .iter()
            .map(|(r, c)| (*r, if *r == a { (c + b) % W } else { *c }))
            .collect();
    } else if let Some(cap) = RE_COL.captures(line) {
        let a: usize = cap[1].parse().unwrap();
        let b: usize = cap[2].parse().unwrap();
        *grid = grid
            .iter()
            .map(|(r, c)| (if *c == a { (r + b) % H } else { *r }, *c))
            .collect();
    } else {
        panic!("Inavlid instr: {}", line);
    }
}

fn lit_pixels(input: &str) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    input
        .lines()
        .for_each(|line| process_instr(&mut result, line));
    result
}

pub fn part1(input: &str) -> usize {
    lit_pixels(input).len()
}

pub fn part2(input: &str) -> String {
    let pix = lit_pixels(input);
    let mut display = vec!["".to_string()];
    for r in 0..H {
        display.push(
            (0..W)
                .map(|c| if pix.contains(&(r, c)) { '#' } else { ' ' })
                .collect(),
        );
    }
    display.push("".to_owned());
    display.join("\n")
}
