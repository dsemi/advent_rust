use crate::utils::parsers::*;

type Switch = fn(&mut [u32]);
type Coord = (usize, usize);

fn parse(
    off: Switch,
    on: Switch,
    toggle: Switch,
    i: &str,
) -> IResult<&str, (Switch, (Coord, Coord))> {
    let (i, f) = alt((
        value(off, tag("turn off ")),
        value(on, tag("turn on ")),
        value(toggle, tag("toggle ")),
    ))(i)?;
    let num = |i| map(u32, |n| n as usize)(i);
    let (i, pts) = separated_pair(coord(num), tag(" through "), coord(num))(i)?;
    Ok((i, (f, pts)))
}

fn run_commands(input: &str, off: Switch, on: Switch, toggle: Switch) -> u32 {
    let mut grid = vec![0; 1000000];
    for line in input.lines() {
        let (f, ((x0, y0), (x1, y1))) = parse(off, on, toggle, line).unwrap().1;
        for x in x0..=x1 {
            let row = 1000 * x;
            f(&mut grid[row + y0..=row + y1]);
        }
    }
    grid.into_iter().sum()
}

pub fn part1(input: &str) -> u32 {
    run_commands(
        input,
        |s| s.fill(0),
        |s| s.fill(1),
        |s| s.iter_mut().for_each(|v| *v ^= 1),
    )
}

pub fn part2(input: &str) -> u32 {
    run_commands(
        input,
        |s| s.iter_mut().for_each(|v| *v = v.saturating_sub(1)),
        |s| s.iter_mut().for_each(|v| *v += 1),
        |s| s.iter_mut().for_each(|v| *v += 2),
    )
}
