use crate::utils::parsers::*;
use num::integer::lcm;
use smallvec::{smallvec as sv, SmallVec};

#[derive(Clone, Eq, PartialEq)]
struct Moon {
    pos: SmallVec<[i64; 3]>,
    vel: SmallVec<[i64; 3]>,
}

fn moon(i: &mut &str) -> PResult<Moon> {
    let (x, y, z) = delimited('<', coord3(preceded((any, '='), i64)), '>').parse_next(i)?;
    let pos = sv![x, y, z];
    let vel = sv![0, 0, 0];
    Ok(Moon { pos, vel })
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            for x in 0..moons[i].vel.len() {
                moons[i].vel[x] += moons[j].pos[x].cmp(&moons[i].pos[x]) as i64;
            }
        }
    }
    for moon in moons.iter_mut() {
        for x in 0..moon.pos.len() {
            moon.pos[x] += moon.vel[x];
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut m = lines(moon).read(input);
    for _ in 0..1000 {
        step(&mut m);
    }
    m.into_iter()
        .map(|v| {
            v.pos.into_iter().map(|x| x.abs()).sum::<i64>()
                * v.vel.into_iter().map(|x| x.abs()).sum::<i64>()
        })
        .sum()
}

pub fn part2(input: &str) -> Option<u64> {
    let moons = lines(moon).read(input);
    (0..=2)
        .map(|n| {
            let mut degen: Vec<_> = moons
                .iter()
                .map(|m| Moon {
                    pos: sv![m.pos[n]],
                    vel: sv![m.vel[n]],
                })
                .collect();
            let mut counter = 1;
            loop {
                step(&mut degen);
                if degen.iter().all(|m| m.vel.iter().all(|&v| v == 0)) {
                    break counter * 2;
                }
                counter += 1;
            }
        })
        .reduce(lcm)
}
