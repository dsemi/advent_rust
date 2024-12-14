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

pub fn part2(input: &str) -> Option<usize> {
    let (mut robots, vels): (Vec<_>, Vec<_>) = input.lines().map(|line| robot.read(line)).unzip();
    let mut grid: Grid<usize, i64> = Grid::new(ROWS, COLS);
    (1..).find(|_| {
        let mut unique = true;
        grid.elems.fill(0);
        robots.iter_mut().zip(&vels).for_each(|(r, &v)| {
            *r += v;
            grid[*r] += 1;
            unique &= grid[*r] == 1;
        });
        unique
    })
}
