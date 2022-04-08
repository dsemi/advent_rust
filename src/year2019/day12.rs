use num::integer::lcm;
use scan_fmt::scan_fmt as scanf;

#[derive(Clone, Eq, PartialEq)]
struct Moon {
    pos: Vec<i64>,
    vel: Vec<i64>,
}

fn parse_moons(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = scanf!(line, "<x={}, y={}, z={}>", i64, i64, i64).unwrap();
            Moon {
                pos: vec![x, y, z],
                vel: vec![0, 0, 0],
            }
        })
        .collect()
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
    let mut m = parse_moons(input);
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

fn find_cycle(moons: Vec<Moon>) -> u64 {
    let mut ms = moons.clone();
    let mut i = 1;
    loop {
        step(&mut ms);
        if ms == moons {
            return i;
        }
        i += 1;
    }
}

pub fn part2(input: &str) -> Option<u64> {
    let moons = parse_moons(input);
    (0..=2)
        .map(|n| {
            find_cycle(
                moons
                    .iter()
                    .map(|m| Moon {
                        pos: vec![m.pos[n]],
                        vel: vec![m.vel[n]],
                    })
                    .collect(),
            )
        })
        .reduce(lcm)
}
