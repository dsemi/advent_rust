use crate::utils::parsers::*;
use Wire::*;
use std::fmt::Write;

const LEN: usize = 729;

#[derive(Clone, Copy)]
enum Wire {
    Num(u16),
    Id(usize),
}

type Op = fn(u16, u16) -> u16;
#[derive(Clone, Copy)]
struct Gate(Op, Wire, Wire);

fn get(graph: &[Gate; LEN], cache: &mut [Option<u16>; LEN], sig: usize) -> u16 {
    if let Some(v) = cache[sig] {
        return v;
    }
    let Gate(f, a, b) = graph[sig];
    let a = match a {
        Num(n) => n,
        Id(i) => get(graph, cache, i),
    };
    let b = match b {
        Num(n) => n,
        Id(i) => get(graph, cache, i),
    };
    let ans = f(a, b);
    cache[sig] = Some(ans);
    ans
}

fn lookup(graph: [Gate; LEN], signal: usize) -> u16 {
    get(&graph, &mut [None; LEN], signal)
}

fn id(i: &mut &str) -> Result<usize> {
    repeat(1..=2, one_of('a'..='z'))
        .fold(|| 0, |acc, v| 27 * acc + (v as u8 - b'a' + 1) as usize)
        .parse_next(i)
}

fn wire(i: &mut &str) -> Result<Wire> {
    alt((u16.map(Num), id.map(Id))).parse_next(i)
}

fn gate(i: &mut &str) -> Result<Gate> {
    let (a, op, b) = alt((
        (wire, " AND ".value((|a, b| a & b) as Op), wire),
        (wire, " OR ".value((|a, b| a | b) as Op), wire),
        (wire, " LSHIFT ".value((|a, b| a << b) as Op), wire),
        (wire, " RSHIFT ".value((|a, b| a >> b) as Op), wire),
        (empty.value(Num(0)), "NOT ".value((|_, b| !b) as Op), wire),
        (empty.value(Num(0)), empty.value((|_, b| b) as Op), wire),
    ))
    .parse_next(i)?;
    Ok(Gate(op, a, b))
}

fn network(i: &str) -> [Gate; LEN] {
    let mut it = iterator(i, terminated((gate, " -> ", id), opt('\n')));
    let mut network = [Gate(|_, b| b, Num(0), Num(0)); LEN];
    it.for_each(|(v, _, k)| network[k] = v);
    assert_eq!("", it.finish().unwrap().0);
    network
}

pub fn part1(input: &str) -> u16 {
    lookup(network(input), 1)
}

pub fn part2(input: &str) -> u16 {
    let mut inp = input.to_string();
    write!(&mut inp, "\n{} -> b", part1(input)).unwrap();
    lookup(network(&inp), 1)
}
