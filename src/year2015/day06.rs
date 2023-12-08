use crate::utils::parsers2::*;

type Switch = fn(&mut [u32]);
type Coord = (usize, usize);

fn parse<'a>(
    off: Switch,
    on: Switch,
    toggle: Switch,
) -> impl Parser<&'a str, (Switch, (Coord, Coord)), ContextError> {
    (
        alt((
            "turn off ".value(off),
            "turn on ".value(on),
            "toggle ".value(toggle),
        )),
        sep_tuple2(coord(usize), "through"),
    )
}

fn run_commands(input: &str, off: Switch, on: Switch, toggle: Switch) -> u32 {
    let mut grid = vec![0; 1000000];
    for line in input.lines() {
        let (f, ((x0, y0), (x1, y1))) = parse(off, on, toggle).read(line);
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
