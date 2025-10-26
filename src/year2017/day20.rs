use crate::utils::parsers::*;
use crate::utils::*;

struct Particle {
    pos: C3<i64>,
    vel: C3<i64>,
    acc: C3<i64>,
}

fn particle(i: &mut &str) -> Result<C3<i64>> {
    delimited((any, "=<"), c3(i64), '>').parse_next(i)
}

fn parse_particles(input: &str) -> impl Iterator<Item = Particle> + '_ {
    input.lines().map(move |line| {
        let (pos, vel, acc) = coord3(particle).read(line);
        Particle { pos, vel, acc }
    })
}

pub fn part1(input: &str) -> Option<usize> {
    itertools::Itertools::position_min_by_key(parse_particles(input), |p| p.acc.abs().sum())
}

pub fn part2(input: &str) -> usize {
    let mut ps = parse_particles(input).collect::<Vec<_>>();
    for _ in 0..100 {
        ps.iter_mut().for_each(|p| {
            p.vel += p.acc;
            p.pos += p.vel;
        });
        let tbl = ps.iter().map(|p| p.pos).counts();
        ps.retain(|p| tbl[&p.pos] == 1);
    }
    ps.len()
}
