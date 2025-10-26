use crate::utils::Cube;
use crate::utils::parsers::*;
use St::*;
use bit_set::BitSet;

fn intersect_volume(cubes: &[Cube<i64>], bs: &[BitSet], cube: &Cube<i64>, set: &BitSet) -> i64 {
    let mut vol = cube.volume();
    for idx in set {
        let common = cube.intersect(&cubes[idx]);
        let inter = set.intersection(&bs[idx]).collect();
        vol -= intersect_volume(cubes, bs, &common, &inter);
    }
    vol
}

#[derive(Clone)]
enum St {
    Off,
    On,
}

fn parse_cube(i: &mut &str) -> ModalResult<(St, Cube<i64>)> {
    let st = terminated(alt(("off".value(Off), "on".value(On))), ' ').parse_next(i)?;
    let ((x0, x1), (y0, y1), (z0, z1)) =
        sep3(preceded((any, '='), sep2(i64, "..")), ',').parse_next(i)?;
    let cube = Cube::new(x0, x1 + 1, y0, y1 + 1, z0, z1 + 1);
    Ok((st, cube))
}

fn solve(input: &str, lo: i64, hi: i64) -> i64 {
    let active_cube = Cube::new(lo, hi, lo, hi, lo, hi);
    let mut cubes = Vec::new();
    let mut on = Vec::new();
    for line in input.lines() {
        let (w, cube) = parse_cube.read(line);
        on.push(matches!(w, On) && cube.intersects(&active_cube));
        cubes.push(cube);
    }
    let mut bs = vec![BitSet::new(); cubes.len()];
    for i in 0..cubes.len() {
        for j in 0..i {
            if cubes[i].intersects(&cubes[j]) {
                bs[j].insert(i);
            }
        }
    }
    let mut ans = 0;
    for i in 0..cubes.len() {
        if !on[i] {
            continue;
        }
        ans += intersect_volume(&cubes, &bs, &cubes[i], &bs[i]);
    }
    ans
}

pub fn part1(input: &str) -> i64 {
    solve(input, -50, 51)
}

pub fn part2(input: &str) -> i64 {
    solve(input, i64::MIN, i64::MAX)
}
