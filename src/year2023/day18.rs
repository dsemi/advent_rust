use crate::utils::parsers::*;
use crate::utils::*;

fn instr1(i: &mut &str) -> Result<(C<i64>, i64)> {
    (
        alt(('U'.value(C(-1, 0)), 'D'.value(C(1, 0)), 'L'.value(C(0, -1)), 'R'.value(C(0, 1)))),
        delimited(' ', i64, delimited(" (#", hex_digit1, ')')),
    )
        .parse_next(i)
}

fn solve<'a>(input: &'a str, instr: impl Parser<&'a str, (C<i64>, i64)>) -> i64 {
    let dig_plan = lines(instr).read(input);
    let pts: Vec<_> = dig_plan
        .iter()
        .scan(C(0, 0), |pos, &(dir, amt)| {
            *pos += dir * amt;
            Some(*pos)
        })
        .collect();
    let area = shoelace(&pts);
    let boundary = dig_plan.iter().map(|(_, amt)| amt).sum();
    let interior = picks_interior(area, boundary);
    boundary + interior
}

pub fn part1(input: &str) -> i64 {
    solve(input, instr1)
}

fn instr2(i: &mut &str) -> Result<(C<i64>, i64)> {
    (alt(('U', 'D', 'L', 'R')), ' ', i64, " (#").parse_next(i)?;
    let amt: u64 = take(5u8).and_then(hex_uint).parse_next(i)?;
    let dir =
        alt(('0'.value(C(0, 1)), '1'.value(C(1, 0)), '2'.value(C(0, -1)), '3'.value(C(-1, 0))))
            .parse_next(i)?;
    ')'.parse_next(i)?;
    Ok((dir, amt as i64))
}

pub fn part2(input: &str) -> i64 {
    solve(input, instr2)
}
