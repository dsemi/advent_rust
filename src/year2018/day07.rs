use ahash::{AHashMap, AHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Scheduler {
    avail: BinaryHeap<Reverse<char>>,
    preds: AHashMap<char, AHashSet<char>>,
    succs: AHashMap<char, AHashSet<char>>,
}

fn parse_steps(input: &str) -> Scheduler {
    let mut preds = AHashMap::new();
    let mut succs = AHashMap::new();
    for line in input.lines() {
        let (a, b) = scan_fmt!(
            line,
            "Step {} must be finished before step {} can begin",
            char,
            char
        )
        .unwrap();
        preds.entry(b).or_insert_with(|| AHashSet::new()).insert(a);
        succs.entry(a).or_insert_with(|| AHashSet::new()).insert(b);
    }
    let mut avail = BinaryHeap::new();
    for c in succs.keys() {
        if !preds.contains_key(c) {
            avail.push(Reverse(*c));
        }
    }
    Scheduler {
        avail: avail,
        preds: preds,
        succs: succs,
    }
}

impl Scheduler {
    fn run(&mut self, mut workers: usize) -> (String, u32) {
        let mut done = AHashSet::new();
        let mut work_queue = BinaryHeap::new();
        let mut result = ("".to_owned(), 0);

        fn sched(
            scheduler: &mut Scheduler,
            work_queue: &mut BinaryHeap<(Reverse<u32>, Reverse<char>)>,
            workers: &mut usize,
            time: u32,
        ) {
            while !scheduler.avail.is_empty() && *workers > 0 {
                *workers -= 1;
                let Reverse(c) = scheduler.avail.pop().unwrap();
                work_queue.push((Reverse(time + c as u32 - 4), Reverse(c)));
            }
        }

        sched(self, &mut work_queue, &mut workers, 0);
        while let Some((Reverse(time), Reverse(curr))) = work_queue.pop() {
            result.0.push(curr);
            result.1 = time;
            workers += 1;
            done.insert(curr);
            for st in self.succs.get(&curr).unwrap_or(&AHashSet::new()) {
                if !done.contains(st) && done.is_superset(&self.preds[st]) {
                    self.avail.push(Reverse(*st));
                }
            }
            sched(self, &mut work_queue, &mut workers, time)
        }
        result
    }
}

pub fn part1(input: &str) -> String {
    parse_steps(input).run(1).0
}

pub fn part2(input: &str) -> u32 {
    parse_steps(input).run(5).1
}
