use num::integer::div_mod_floor;
use std::cmp::max;
use std::collections::HashMap;

type Reactions<'a> = HashMap<&'a str, (i64, Vec<(i64, &'a str)>)>;

fn parse_reactions<'a>(input: &'a str) -> Reactions<'a> {
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

fn num_ore<'a>(reactions: &Reactions<'a>, n: i64) -> i64 {
    fn go<'a>(
        reactions: &Reactions<'a>,
        surplus: &mut HashMap<&'a str, i64>,
        ore: &mut i64,
        k: &'a str,
        c: i64,
    ) {
        if reactions.contains_key(&k) {
            let (n, chems) = &reactions[&k];
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
                *surplus.entry(&k).or_insert(0) += n - r;
            }
        } else {
            *ore += c;
        }
    }
    let mut ore = 0;
    let mut surplus = HashMap::new();
    go(reactions, &mut surplus, &mut ore, "FUEL", n);
    ore
}

pub fn part1(input: &str) -> i64 {
    num_ore(&parse_reactions(input), 1)
}

const TRILLION: i64 = 1_000_000_000_000;

pub fn part2(input: &str) -> i64 {
    let reactions = parse_reactions(input);
    let (mut a, mut b) = (0, TRILLION);
    while a < b {
        let mid = (a + b) / 2;
        if num_ore(&reactions, mid) > TRILLION {
            b = mid - 1;
        } else {
            a = mid + 1;
        }
    }
    if num_ore(&reactions, a) > TRILLION {
        a - 1
    } else {
        a
    }
}
