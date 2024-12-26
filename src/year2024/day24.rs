use crate::utils::parsers::*;
use hashbrown::HashSet;
use Op::*;
use Wire::*;

const LEN: usize = 1 << 15;

#[derive(Clone, Copy)]
enum Wire {
    Num(u8),
    Id(usize),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy)]
struct Gate<T>(Op, T, T);

fn get(graph: &[Option<Gate<Wire>>], cache: &mut [Option<u8>], sig: usize) -> Option<u8> {
    if let Some(v) = cache[sig] {
        return Some(v);
    }
    let Gate(op, a, b) = graph[sig]?;
    let a = match a {
        Num(n) => n,
        Id(i) => get(graph, cache, i)?,
    };
    let b = match b {
        Num(n) => n,
        Id(i) => get(graph, cache, i)?,
    };
    let ans = match op {
        And => a & b,
        Or => a | b,
        Xor => a ^ b,
    };
    cache[sig] = Some(ans);
    Some(ans)
}

fn id(k: &str) -> usize {
    k.bytes().fold(0, |acc, v| acc << 5 | ((v - b'0') as usize & 31))
}

fn gate<'a>(i: &mut &'a str) -> PResult<Gate<&'a str>> {
    let (a, op, b) = (
        alphanumeric1,
        alt((" AND ".value(And), " OR ".value(Or), " XOR ".value(Xor))),
        alphanumeric1,
    )
        .parse_next(i)?;
    Ok(Gate(op, a, b))
}

pub fn part1(input: &str) -> u64 {
    let (vals, gates) = input.split_once("\n\n").unwrap();
    let mut network = vec![None; LEN];
    let mut it = iterator(vals, terminated((alphanumeric1.map(id), ": ", u8), opt('\n')));
    it.for_each(|(k, _, n)| network[k] = Some(Gate(And, Num(n), Num(n))));
    assert_eq!("", it.finish().unwrap().0);
    let mut it = iterator(gates, terminated((gate, " -> ", alphanumeric1.map(id)), opt('\n')));
    it.for_each(|(Gate(op, a, b), _, k)| network[k] = Some(Gate(op, Id(id(a)), Id(id(b)))));
    assert_eq!("", it.finish().unwrap().0);
    let cache = &mut vec![None; LEN];
    (id("z00")..id("z64"))
        .rev()
        .filter_map(|k| get(&network, cache, k))
        .fold(0, |acc, b| acc << 1 | b as u64)
}

pub fn part2(input: &str) -> String {
    let gates = lines((gate, " -> ", alphanumeric1)).read(input.split_once("\n\n").unwrap().1);
    let mut lookup = HashSet::new();
    let mut swapped = HashSet::new();
    for &(Gate(op, left, right), _, _) in &gates {
        lookup.insert((left, op));
        lookup.insert((right, op));
    }
    for &(Gate(op, left, right), _, to) in &gates {
        match op {
            And if left != "x00" && right != "x00" && !lookup.contains(&(to, Or)) => {
                swapped.insert(to);
            }
            Or if to.starts_with('z') && to != "z45" => {
                swapped.insert(to);
            }
            Or if lookup.contains(&(to, Or)) => {
                swapped.insert(to);
            }
            Xor if left.starts_with('x') || right.starts_with('x') => {
                if left != "x00" && right != "x00" && !lookup.contains(&(to, Xor)) {
                    swapped.insert(to);
                }
            }
            Xor if !to.starts_with('z') => {
                swapped.insert(to);
            }
            _ => (),
        }
    }
    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}
