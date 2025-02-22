use super::intcode;
use crate::utils::*;

fn parse_grid(input: Vec<i64>) -> Grid<char, i64> {
    input.into_iter().map(|x| x as u8 as char).collect()
}

fn is_scaffold(grid: &Grid<char, i64>, pos: C<i64>) -> bool {
    grid.get(pos).is_some_and(|&c| "#^<>v".contains(c))
}

pub fn part1(input: &str) -> i64 {
    let mut prog = intcode::new(input);
    prog.run();
    let ins = prog.output.drain(..).collect();
    let grid = parse_grid(ins);
    grid.idxs()
        .filter_map(|C(y, x)| {
            [C(y, x), C(y, x + 1), C(y + 1, x), C(y, x - 1), C(y - 1, x)]
                .into_iter()
                .all(|pos| is_scaffold(&grid, pos))
                .then_some(x * y)
        })
        .sum()
}

fn keep_moving(grid: &Grid<char, i64>, pos: C<i64>, c: &str, d: C<i64>) -> Vec<String> {
    let mut p = pos + d;
    if !is_scaffold(grid, p) {
        return vec![];
    }
    while is_scaffold(grid, p + d) {
        p += d;
    }
    let mut result = vec![c.to_string(), pos.dist(&p).to_string()];
    result.extend(go(grid, p, d));
    result
}

fn go(grid: &Grid<char, i64>, pos: C<i64>, C(x, y): C<i64>) -> Vec<String> {
    keep_moving(grid, pos, "L", C(-y, x))
        .into_iter()
        .chain(keep_moving(grid, pos, "R", C(y, -x)))
        .collect()
}

fn find_path(grid: &Grid<char, i64>) -> Vec<String> {
    let (pos, dir) = grid.idx_iter().find(|&(_, &v)| "^><v".contains(v)).unwrap();
    let res = go(
        grid,
        pos,
        match dir {
            '^' => C(-1, 0),
            'v' => C(1, 0),
            '<' => C(0, -1),
            '>' => C(0, 1),
            _ => panic!("Bad dir: {}", dir),
        },
    );
    res.chunks(2).map(|ch| ch.join(",")).collect()
}

fn splits(x: &[String], s: &[String]) -> Vec<Vec<String>> {
    let mut i = 0;
    let mut c = 0;
    let mut result: Vec<Vec<String>> = Vec::new();
    while s.len() <= x.len() && i <= x.len() - s.len() {
        if x[i..i + s.len()].to_vec() == s {
            result.push(x[c..i].to_vec());
            i += s.len();
            c = i;
            continue;
        }
        i += 1;
    }
    result.push(x[c..].to_vec());
    result
}

fn go2(xs: Vec<Vec<String>>, fns: i64) -> Option<Vec<Vec<String>>> {
    if xs.is_empty() {
        return Some(vec![]);
    }
    if fns > 0 {
        for i in 1..=xs[0].len() {
            let candidate = &xs[0][..i].to_vec();
            let fragments = xs
                .iter()
                .flat_map(|x| splits(x, candidate).into_iter().filter(|y| !y.is_empty()))
                .collect();
            if let Some(res) = go2(fragments, fns - 1) {
                return Some(std::iter::once(candidate.clone()).chain(res).collect());
            }
        }
    }
    None
}

fn compress(instrs: Vec<String>) -> Vec<Vec<String>> {
    let repl_map = vec![vec!["A".to_string()], vec!["B".to_string()], vec!["C".to_string()]]
        .into_iter()
        .zip(go2(vec![instrs.clone()], 3).unwrap())
        .collect::<Vec<_>>();
    std::iter::once(repl_map.iter().fold(instrs, |a, b| {
        itertools::Itertools::intersperse(splits(&a, &b.1).into_iter(), b.0.clone())
            .flatten()
            .collect()
    }))
    .chain(repl_map.into_iter().map(|x| x.1))
    .collect()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut prog = intcode::new(input);
    prog.run();
    let ins = prog.output.drain(..).collect();
    let grid = parse_grid(ins);
    let path = find_path(&grid);
    let comped = compress(path);
    let mut inps = comped
        .into_iter()
        .map(|x| x.join(","))
        .chain(vec!["n".to_string()])
        .collect::<Vec<_>>()
        .join("\n");
    inps.push('\n');
    prog = intcode::new(input);
    prog[0] = 2;
    for c in inps.chars() {
        prog.input.push_back(c as i64);
    }
    prog.run();
    prog.output.drain(..).last()
}
