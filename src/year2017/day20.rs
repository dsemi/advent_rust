use crate::utils::parsers::*;
use crate::utils::C3;
use counter::Counter;
use itertools::Itertools;

struct Particle {
    pos: C3<i64>,
    vel: C3<i64>,
    acc: C3<i64>,
}

fn particle(i: &str) -> IResult<&str, C3<i64>> {
    let (i, x) = preceded(pair(anychar, tag("=<")), i64)(i)?;
    let (i, y) = preceded(tag(","), i64)(i)?;
    let (i, z) = delimited(tag(","), i64, tag(">"))(i)?;
    Ok((i, C3(x, y, z)))
}

fn parse_particles(input: &str) -> impl Iterator<Item = Particle> + '_ {
    input.lines().map(move |line| {
        let cs = list(particle)(line).unwrap().1;
        Particle {
            pos: cs[0],
            vel: cs[1],
            acc: cs[2],
        }
    })
}

pub fn part1(input: &str) -> Option<usize> {
    parse_particles(input).position_min_by_key(|p| p.acc.abs().sum())
}

pub fn part2(input: &str) -> usize {
    let mut ps = parse_particles(input).collect::<Vec<_>>();
    for _ in 0..100 {
        ps.iter_mut().for_each(|p| {
            p.vel += p.acc;
            p.pos += p.vel;
        });
        let tbl = ps.iter().map(|p| p.pos).collect::<Counter<_>>();
        ps.retain(|p| tbl[&p.pos] == 1);
    }
    ps.len()
}
