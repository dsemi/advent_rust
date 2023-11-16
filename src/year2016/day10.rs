use ahash::AHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i64, space1, u8};
use nom::combinator::recognize;
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;
use std::cmp::{max, min};
use Src::*;

enum Src<'a> {
    Value(i64),
    Bot(&'a str, fn(i64, i64) -> i64),
}

type Node<'a> = Vec<Src<'a>>;

fn populate_ins<'a>(m: &mut AHashMap<&'a str, Vec<i64>>, t: &AHashMap<&str, Node<'a>>, k: &'a str) {
    if m.contains_key(&k) {
        return;
    }
    let mut inps: Vec<i64> = t[k]
        .iter()
        .map(|src| match src {
            Value(v) => *v,
            Bot(b, f) => {
                populate_ins(m, t, b);
                m[b].iter().copied().reduce(f).unwrap()
            }
        })
        .collect();
    inps.sort_unstable();
    m.insert(k, inps);
}

fn bot(i: &str) -> IResult<&str, Vec<(&str, Src<'_>)>> {
    let loc = |i| recognize(separated_pair(alpha1, space1, u8))(i);
    let (i, name) = loc(i)?;
    let (i, lo) = preceded(tag(" gives low to "), loc)(i)?;
    let (i, hi) = preceded(tag(" and high to "), loc)(i)?;
    Ok((i, vec![(lo, Bot(name, min)), (hi, Bot(name, max))]))
}

fn value(i: &str) -> IResult<&str, Vec<(&str, Src<'_>)>> {
    let (i, val) = preceded(tag("value "), i64)(i)?;
    let (i, b) = preceded(tag(" goes to "), recognize(pair(tag("bot "), u8)))(i)?;
    Ok((i, vec![(b, Value(val))]))
}

fn run_factory(input: &str) -> AHashMap<&str, Vec<i64>> {
    let mut tbl: AHashMap<&str, Node> = AHashMap::new();
    for line in input.lines() {
        for (k, src) in alt((value, bot))(line).unwrap().1 {
            tbl.entry(k).or_default().push(src);
        }
    }
    let mut result: AHashMap<&str, Vec<i64>> = AHashMap::new();
    for k in tbl.keys() {
        populate_ins(&mut result, &tbl, k);
    }
    result
}

pub fn part1(input: &str) -> Option<String> {
    run_factory(input)
        .into_iter()
        .filter(|(_k, v)| v == &vec![17, 61])
        .map(|x| x.0.rsplit_once(' ').unwrap().1.to_string())
        .next()
}

pub fn part2(input: &str) -> i64 {
    let m = run_factory(input);
    m["output 0"][0] * m["output 1"][0] * m["output 2"][0]
}
