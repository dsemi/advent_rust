use crate::utils::parsers::*;
use ahash::AHashMap;

fn parse<'a>(reg: &AHashMap<&str, i64>, i: &'a str) -> (bool, &'a str, i64) {
    let mut p = |i| {
        let (i, mut_r) = terminated(alpha1, space1)(i)?;
        let (i, sgn) = alt((value(1, tag("inc")), value(-1, tag("dec"))))(i)?;
        let (i, mut_n) = delimited(space1, i64, tag(" if "))(i)?;
        let (i, cmp_r) = terminated(alpha1, space1)(i)?;
        let rv = *reg.get(cmp_r).unwrap_or(&0);
        let (i, cond) = map_res(
            separated_pair(take_while1(|c| c != ' '), space1, i64),
            |(cmp, cmp_n)| match cmp {
                "==" => Ok(rv == cmp_n),
                "!=" => Ok(rv != cmp_n),
                ">" => Ok(rv > cmp_n),
                ">=" => Ok(rv >= cmp_n),
                "<" => Ok(rv < cmp_n),
                "<=" => Ok(rv <= cmp_n),
                _ => Err("Parse error"),
            },
        )(i)?;
        Ok((i, (cond, mut_r, sgn * mut_n)))
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
