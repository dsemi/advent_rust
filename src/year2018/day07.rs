use ahash::{AHashMap, AHashSet};
use scan_fmt::scan_fmt as scanf;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Scheduler {
    avail: BinaryHeap<Reverse<char>>,
    preds: AHashMap<char, AHashSet<char>>,
    succs: AHashMap<char, AHashSet<char>>,
    workers: usize,
}

fn parse_steps(input: &str, workers: usize) -> Scheduler {
    let mut preds = AHashMap::new();
    let mut succs = AHashMap::new();
    for line in input.lines() {
        let (a, b) = scanf!(
            line,
            "Step {} must be finished before step {} can begin",
            char,
            char
        )
        .unwrap();
        preds.entry(b).or_insert_with(AHashSet::new).insert(a);
        succs.entry(a).or_insert_with(AHashSet::new).insert(b);
    }
    Scheduler {
        avail: succs
            .keys()
            .filter_map(|c| (!preds.contains_key(c)).then(|| Reverse(*c)))
            .collect(),
        preds,
        succs,
        workers,
    }
}

impl Scheduler {
    fn run(&mut self) -> (String, u32) {
        let mut done = AHashSet::new();
        let mut work_queue = BinaryHeap::new();
        let mut result = ("".to_owned(), 0);

        loop {
            while !self.avail.is_empty() && work_queue.len() < self.workers {
                let Reverse(c) = self.avail.pop().unwrap();
                work_queue.push((Reverse(result.1 + c as u32 - 4), Reverse(c)));
            }
            if let Some((Reverse(time), Reverse(curr))) = work_queue.pop() {
                result.0.push(curr);
                result.1 = time;
                done.insert(curr);
                for st in self.succs.get(&curr).unwrap_or(&AHashSet::new()) {
                    if done.is_superset(&self.preds[st]) {
                        self.avail.push(Reverse(*st));
                    }
                }
            } else {
                break;
            }
        }
        result
    }
}

pub fn part1(input: &str) -> String {
    parse_steps(input, 1).run().0
}

pub fn part2(input: &str) -> u32 {
    parse_steps(input, 5).run().1
}
