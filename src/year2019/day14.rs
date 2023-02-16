use crate::utils::partition_point;
use ahash::AHashMap;
use num::integer::div_mod_floor;
use std::cmp::max;

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
    fn go<'a>(
        reactions: &Reactions<'a>,
        surplus: &mut AHashMap<&'a str, i64>,
        ore: &mut i64,
        k: &'a str,
        c: i64,
    ) {
        if let Some((n, chems)) = reactions.get(&k) {
            let (q, r) = div_mod_floor(c, *n);
            for (a, chem) in chems.iter() {
                let amt = a * if r != 0 { q + 1 } else { q };
                let val = *surplus.get(chem).unwrap_or(&0);
                surplus.insert(chem, max(0, val - amt));
                if amt > val {
                    go(reactions, surplus, ore, chem, amt - val);
                }
            }
            if r != 0 {
                *surplus.entry(k).or_insert(0) += n - r;
            }
        } else {
            *ore += c;
        }
    }
    let mut ore = 0;
    let mut surplus = AHashMap::new();
    go(reactions, &mut surplus, &mut ore, "FUEL", n);
    ore
}

pub fn part1(input: &str) -> i64 {
    num_ore(&parse_reactions(input), 1)
}

const TRILLION: i64 = 1_000_000_000_000;

pub fn part2(input: &str) -> i64 {
    let reactions = parse_reactions(input);
    partition_point(0, TRILLION, |&fuel| num_ore(&reactions, fuel) <= TRILLION) - 1
}
