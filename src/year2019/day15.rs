use crate::utils::*;
use crate::year2019::intcode;

fn search(prog: intcode::Program) -> impl Iterator<Item = (usize, bool, intcode::Program)> {
    let start = (0, 0);
    let frontier = vec![(0, (false, start, prog))]
        .into_iter()
        .collect();
    let visited = vec![start].into_iter().collect();
    Bfs {
        frontier,
        visited,
        hash: |x: &(bool, (i64, i64), intcode::Program)| x.1,
        neighbs: |(_, st, p): &(bool, (i64, i64), intcode::Program)| {
            [(0, 1), (0, -1), (-1, 0), (1, 0)]
                .iter()
                .enumerate()
                .filter_map(|(i, dir)| {
                    let st2 = (st.0 + dir.0, st.1 + dir.1);
                    let n = i as i64 + 1;
                    let mut p2 = p.clone();
                    p2.input.push_back(n);
                    p2.run();
                    let out = p2.output.drain(..).collect::<Vec<_>>();
                    assert!(out.len() == 1, "Too many outputs");
                    (out[0] == 1 || out[0] == 2).then(|| (out[0] == 2, st2, p2))
                })
                .collect::<Vec<_>>()
        },
    }
    .map(|(d, (b, _, p))| (d, b, p))
}

fn find_oxygen(prog: intcode::Program) -> Option<(usize, intcode::Program)> {
    search(prog)
        .filter_map(|(d, b, p)| b.then(|| (d, p)))
        .next()
}

pub fn part1(input: &str) -> Option<usize> {
    find_oxygen(intcode::new(input)).map(|x| x.0)
}

pub fn part2(input: &str) -> Option<usize> {
    let prog = find_oxygen(intcode::new(input)).map(|x| x.1)?;
    search(prog).last().map(|x| x.0)
}
