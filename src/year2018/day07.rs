use crate::utils::parsers::*;
use crate::utils::*;
use std::collections::BinaryHeap;

fn solve(input: &str, workers: usize) -> (String, i32) {
    let mut conns = [(0, 0); 26];
    for line in input.lines() {
        let (a, b) = (
            preceded("Step ", any),
            delimited(" must be finished before step ", any, " can begin."),
        )
            .read(line);
        let (a, b) = (b'Z' - a as u8, b'Z' - b as u8);
        conns[b as usize].0 |= 1 << a;
        conns[a as usize].1 |= 1 << b;
    }
    let mut avail = conns
        .iter()
        .enumerate()
        .filter_map(|(i, &(inc, out))| (inc == 0 && out != 0).then_some(i))
        .collect::<BinaryHeap<_>>();
    let mut work_queue = BinaryHeap::new();
    let mut result = ("".to_owned(), 0);
    let fill = |avail: &mut BinaryHeap<usize>, q: &mut BinaryHeap<(i32, usize)>, t: i32| {
        while !avail.is_empty() && q.len() < workers {
            let c = avail.pop().unwrap();
            q.push((t - 86 + c as i32, c));
        }
    };
    fill(&mut avail, &mut work_queue, 0);
    while let Some((time, curr)) = work_queue.pop() {
        result.0.push((b'Z' - curr as u8) as char);
        result.1 = -time;
        for st in bits(conns[curr].1) {
            conns[st].0 &= !(1 << curr);
            if conns[st].0 == 0 {
                avail.push(st);
            }
        }
        fill(&mut avail, &mut work_queue, -result.1);
    }
    result
}

pub fn part1(input: &str) -> String {
    solve(input, 1).0
}

pub fn part2(input: &str) -> i32 {
    solve(input, 5).1
}
