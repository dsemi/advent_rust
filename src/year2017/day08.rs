use crate::utils::parsers2::*;
use ahash::AHashMap;

fn parse<'a>(reg: &AHashMap<&str, i64>, i: &'a str) -> (bool, &'a str, i64) {
    let mut p = |i: &mut &'a str| {
        let (mut_r, sgn, mut_n, _, cmp_r) = (
            alpha1,
            alt((" inc ".value(1), " dec ".value(-1))),
            i64,
            " if ",
            alpha1,
        )
            .parse_next(i)?;
        let rv = *reg.get(cmp_r).unwrap_or(&0);
        let cond = alt((
            preceded(" == ", i64).map(|cmp_n| rv == cmp_n),
            preceded(" != ", i64).map(|cmp_n| rv != cmp_n),
            preceded(" > ", i64).map(|cmp_n| rv > cmp_n),
            preceded(" >= ", i64).map(|cmp_n| rv >= cmp_n),
            preceded(" < ", i64).map(|cmp_n| rv < cmp_n),
            preceded(" <= ", i64).map(|cmp_n| rv <= cmp_n),
        ))
        .parse_next(i)?;
        Ok((cond, mut_r, sgn * mut_n))
    };
    p.read(i)
}

fn run_cmd<'a>(reg: &mut AHashMap<&'a str, i64>, line: &'a str) -> i64 {
    let (cond, mut_r, mut_n) = parse(reg, line);
    if cond {
        *reg.entry(mut_r).or_insert(0) += mut_n;
    }
    *reg.get(mut_r).unwrap_or(&0)
}

pub fn part1(input: &str) -> Option<i64> {
    let mut tbl = AHashMap::new();
    input.lines().for_each(|line| {
        run_cmd(&mut tbl, line);
    });
    tbl.into_iter().map(|x| x.1).max()
}

pub fn part2(input: &str) -> Option<i64> {
    let mut tbl = AHashMap::new();
    input.lines().map(|line| run_cmd(&mut tbl, line)).max()
}
