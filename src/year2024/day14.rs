use crate::utils::parsers::*;
use crate::utils::*;
use std::iter::zip;

const ROWS: i64 = 103;
const COLS: i64 = 101;

fn robot(i: &mut &str) -> Result<(C<Mod<ROWS>, Mod<COLS>>, C<Mod<ROWS>, Mod<COLS>>)> {
    let ((px, py), (vx, vy)) = preceded("p=", sep2(coord(i64), "v=")).parse_next(i)?;
    Ok((C(Mod(py), Mod(px)), C(Mod(vy), Mod(vx))))
}

fn cmp(a: i64, b: i64) -> Option<usize> {
    (a != b).then_some((a.cmp(&b) as usize).wrapping_add(1) / 2)
}

pub fn part1(input: &str) -> usize {
    let (mut robots, vels): (Vec<_>, Vec<_>) = input.lines().map(|line| robot.read(line)).unzip();
    zip(&mut robots, &vels).for_each(|(r, &v)| *r += v * 100);
    let mut qs = [0; 4];
    robots
        .into_iter()
        .filter_map(|C(Mod(r), Mod(c))| Some(cmp(r, ROWS / 2)? + 2 * cmp(c, COLS / 2)?))
        .for_each(|q| qs[q] += 1);
    qs.into_iter().product()
}

fn ivar(i: impl Iterator<Item = i64>) -> i64 {
    let (n, sum, sum_of_squares) = i.fold((0, 0, 0), |(n, s, sos), v| (n + 1, s + v, sos + v * v));
    sum_of_squares / n - (sum * sum) / (n * n)
}

pub fn part2(input: &str) -> Option<i64> {
    let (rbs, vels): (Vec<_>, Vec<_>) = input.lines().map(|line| robot.read(line)).unzip();
    let rvar = ivar(rbs.iter().map(|r| r.0.0));
    let rt = (1..).find(|&t| rvar / 2 > ivar(zip(&rbs, &vels).map(|(r, v)| (r.0 + v.0 * t).0)))?;
    let cvar = ivar(rbs.iter().map(|r| r.1.0));
    let ct = (1..).find(|&t| cvar / 2 > ivar(zip(&rbs, &vels).map(|(r, v)| (r.1 + v.1 * t).0)))?;
    Some((Mod::<ROWS>(COLS).mod_inv() * (rt - ct)).0 * COLS + ct)
}
