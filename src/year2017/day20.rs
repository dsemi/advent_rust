use counter::Counter;
use itertools::Itertools;
use regex::Regex;

use crate::utils::Coord3;

struct Particle {
    pos: Coord3<i64>,
    vel: Coord3<i64>,
    acc: Coord3<i64>,
}

fn parse_particles(input: &str) -> impl Iterator<Item = Particle> + '_ {
    let reg = Regex::new(r"-?\d+").unwrap();
    input.lines().map(move |line| {
        let cs: Vec<Coord3<i64>> = line
            .split(", ")
            .map(|comp| {
                let ds: Vec<i64> = reg
                    .find_iter(comp)
                    .map(|x| x.as_str().parse().unwrap())
                    .collect();
                Coord3::new(ds[0], ds[1], ds[2])
            })
            .collect();
        Particle {
            pos: cs[0],
            vel: cs[1],
            acc: cs[2],
        }
    })
}

pub fn part1(input: &str) -> Option<usize> {
    parse_particles(input).position_min_by_key(|p| p.acc.x.abs() + p.acc.y.abs() + p.acc.z.abs())
}

pub fn part2(input: &str) -> usize {
    let mut ps = parse_particles(input).collect::<Vec<_>>();
    for _ in 0..100 {
        for p in ps.iter_mut() {
            p.vel += p.acc;
            p.pos += p.vel;
        }
        let tbl = ps.iter().map(|p| p.pos).collect::<Counter<_>>();
        ps.retain(|p| tbl[&p.pos] == 1);
    }
    ps.len()
}
