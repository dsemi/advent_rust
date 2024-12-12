use crate::utils::parsers::*;
use crate::utils::partition_point;
use ahash::AHashMap;
use genawaiter;

struct Reactions<'a> {
    #[expect(clippy::type_complexity)]
    graph: AHashMap<&'a str, (i64, Vec<(i64, &'a str)>)>,
    topo: Vec<&'a str>,
}

fn chemical<'a>(i: &mut &'a str) -> PResult<(i64, &'a str)> {
    separated_pair(i64, space1, alpha1).parse_next(i)
}

#[expect(clippy::type_complexity)]
fn parse<'a>(i: &mut &'a str) -> PResult<(&'a str, (i64, Vec<(i64, &'a str)>))> {
    separated_pair(list(chemical), " => ", chemical)
        .map(|(srcs, (n, out))| (out, (n, srcs)))
        .parse_next(i)
}

fn parse_reactions(input: &str) -> Reactions {
    let graph = lines_iter(input, parse).collect::<AHashMap<_, _>>();
    let mut incoming = AHashMap::new();
    graph.values().for_each(|(_, srcs)| {
        srcs.iter().for_each(|(_, src)| *incoming.entry(src).or_insert(0) += 1)
    });
    let mut topo = Vec::new();
    let mut no_incoming = vec!["FUEL"];
    while let Some(e) = no_incoming.pop() {
        if let Some((_, srcs)) = graph.get(e) {
            topo.push(e);
            srcs.iter().for_each(|(_, m)| {
                *incoming.get_mut(m).unwrap() -= 1;
                if incoming[m] == 0 {
                    no_incoming.push(m);
                }
            })
        }
    }

    Reactions { graph, topo }
}

fn num_ore(reactions: &Reactions, n: i64) -> i64 {
    let mut cnts: AHashMap<&str, i64> = vec![("FUEL", n)].into_iter().collect();
    reactions.topo.iter().for_each(|e| {
        let (n, srcs) = &reactions.graph[e];
        // Num reactions required to produce n of e.
        let k = (cnts[e] + n - 1) / n;
        srcs.iter().for_each(|(n, m)| *cnts.entry(m).or_insert(0) += k * n);
    });
    cnts["ORE"]
}

pub fn part1(input: &str) -> i64 {
    num_ore(&parse_reactions(input), 1)
}

const TRILLION: i64 = 1_000_000_000_000;

pub fn part2(input: &str) -> i64 {
    let reactions = parse_reactions(input);
    partition_point(0, TRILLION, |&fuel| num_ore(&reactions, fuel) <= TRILLION) - 1
}
