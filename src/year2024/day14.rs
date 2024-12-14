use crate::utils::parsers::*;
use crate::utils::*;

const ROWS: i64 = 103;
const COLS: i64 = 101;

fn robot(i: &mut &str) -> PResult<(C<Mod<ROWS>, Mod<COLS>>, C<Mod<ROWS>, Mod<COLS>>)> {
    let ((px, py), (vx, vy)) = preceded("p=", sep2(coord(i64), "v=")).parse_next(i)?;
    Ok((C(Mod(py), Mod(px)), C(Mod(vy), Mod(vx))))
}

fn cmp(a: i64, b: i64) -> Option<usize> {
    (a != b).then_some((a.cmp(&b) as usize).wrapping_add(1) / 2)
}

pub fn part1(input: &str) -> usize {
    let (mut robots, vels): (Vec<_>, Vec<_>) = input.lines().map(|line| robot.read(line)).unzip();
    (0..100).for_each(|_| robots.iter_mut().zip(&vels).for_each(|(r, &v)| *r += v));
    let mut qs = [0; 4];
    robots
        .into_iter()
        .filter_map(|C(Mod(r), Mod(c))| Some(cmp(r, ROWS / 2)? + 2 * cmp(c, COLS / 2)?))
        .for_each(|q| qs[q] += 1);
    qs.into_iter().product()
}

pub fn part2(input: &str) -> Option<i64> {
    let (robots, vels): (Vec<_>, Vec<_>) = input.lines().map(|line| robot.read(line)).unzip();
    let rt = (1..).find(|&t| {
        let mut rs = [0; ROWS as usize];
        robots.iter().zip(&vels).for_each(|(r, v)| rs[(r.0 + v.0 * t).0 as usize] += 1);
        rs.into_iter().filter(|&r| r >= 30).count() >= 2
    })?;
    let ct = (1..).find(|&t| {
        let mut cs = [0; COLS as usize];
        robots.iter().zip(&vels).for_each(|(r, v)| cs[(r.1 + v.1 * t).0 as usize] += 1);
        cs.into_iter().filter(|&c| c >= 30).count() >= 2
    })?;
    Some((Mod::<ROWS>(COLS).mod_inv() * (rt - ct)).0 * COLS + ct)
}
