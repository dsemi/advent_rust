use crate::utils::parsers::*;

fn parser<'i>(p: i64) -> impl Parser<&'i str, i64> {
    move |i: &mut &str| {
        expression(alt((i64, delimited('(', parser(p), ')'))))
            .infix(delimited(
                space0,
                dispatch! {any;
                    '+' => Infix::Left(p, |_, a, b| Ok(a + b)),
                    '*' => Infix::Left(1, |_, a, b| Ok(a * b)),
                    '\n' => Infix::Left(0, |_, a, b| Ok(a + b)),
                    _ => fail,
                },
                space0,
            ))
            .parse_next(i)
    }
}

pub fn part1(input: &str) -> i64 {
    parser(1).read(input)
}

pub fn part2(input: &str) -> i64 {
    parser(2).read(input)
}
