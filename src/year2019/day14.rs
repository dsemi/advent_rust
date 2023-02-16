use crate::utils::partition_point;
use ahash::AHashMap;

type Reactions<'a> = AHashMap<&'a str, (i64, Vec<(i64, &'a str)>)>;

fn parse_reactions(input: &str) -> Reactions {
    input
        .lines()
        .map(|line| {
            let pts = line.split(" => ").collect::<Vec<_>>();
            let inps = pts[0]
                .split(", ")
                .map(|inp| {
                    let pts2 = inp.split_whitespace().collect::<Vec<_>>();
                    (pts2[0].parse().unwrap(), pts2[1])
                })
                .collect();
            let outp = pts[1].split_whitespace().collect::<Vec<_>>();
            (outp[1], (outp[0].parse().unwrap(), inps))
        })
        .collect()
}

fn num_ore(reactions: &Reactions, n: i64) -> i64 {
    let mut incoming = AHashMap::new();
    reactions.values().for_each(|(_, srcs)| {
        srcs.iter()
            .for_each(|(_, src)| *incoming.entry(src).or_insert(0) += 1)
    });
    let mut topo = Vec::new();
    let mut no_incoming = vec!["FUEL"];
    while let Some(e) = no_incoming.pop() {
        if let Some((_, srcs)) = reactions.get(e) {
            topo.push(e);
            srcs.iter().for_each(|(_, m)| {
                *incoming.get_mut(m).unwrap() -= 1;
                if incoming[m] == 0 {
                    no_incoming.push(m);
                }
            })
        }
    }
    let mut cnts: AHashMap<&str, i64> = vec![("FUEL", n)].into_iter().collect();
    topo.iter().for_each(|e| {
        let (n, srcs) = &reactions[e];
        let k = (cnts[e] + n - 1) / n;
        *cnts.get_mut(e).unwrap() -= n * k;
        srcs.iter()
            .for_each(|(n, m)| *cnts.entry(m).or_insert(0) += k * n);
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
