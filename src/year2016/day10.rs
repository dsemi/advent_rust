use ahash::AHashMap;
use std::cmp::{max, min};
use Src::*;

enum Src {
    Value(i64),
    Bot(String, fn(i64, i64) -> i64),
}

type Node = Vec<Src>;

fn populate_ins(m: &mut AHashMap<String, Vec<i64>>, tbl: &AHashMap<String, Node>, k: &str) {
    if m.contains_key(k) {
        return;
    }
    let mut inps: Vec<i64> = tbl[k]
        .iter()
        .map(|src| match src {
            Value(v) => *v,
            Bot(b, f) => {
                populate_ins(m, tbl, b);
                m[b].iter().copied().reduce(f).unwrap()
            }
        })
        .collect();
    inps.sort_unstable();
    m.insert(k.to_string(), inps);
}

fn run_factory(input: &str) -> AHashMap<String, Vec<i64>> {
    let mut tbl: AHashMap<String, Node> = AHashMap::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["bot", n, _, _, _, o1, n1, _, _, _, o2, n2] => {
                let e = tbl.entry(format!("{} {}", o1, n1)).or_insert_with(Vec::new);
                e.push(Bot(format!("bot {}", n), min));
                let e = tbl.entry(format!("{} {}", o2, n2)).or_insert_with(Vec::new);
                e.push(Bot(format!("bot {}", n), max));
            }
            ["value", v, _, _, o, n] => {
                let e = tbl.entry(format!("{} {}", o, n)).or_insert_with(Vec::new);
                e.push(Value(v.parse().unwrap()));
            }
            _ => panic!("Parse failed: {}", line),
        }
    }
    let mut result: AHashMap<String, Vec<i64>> = AHashMap::new();
    for k in tbl.keys() {
        populate_ins(&mut result, &tbl, k);
    }
    result
}

pub fn part1(input: &str) -> Option<String> {
    run_factory(input)
        .into_iter()
        .filter(|(_k, v)| v == &vec![17, 61])
        .map(|x| x.0.rsplit_once(" ").unwrap().1.to_string())
        .next()
}

pub fn part2(input: &str) -> i64 {
    let m = run_factory(input);
    m["output 0"][0] * m["output 1"][0] * m["output 2"][0]
}
