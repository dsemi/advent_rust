use nom::branch::alt;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::cond;
use nom::combinator::opt;
use nom::combinator::verify;
use nom::map_res;
use nom::named;
use nom::recognize;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use std::str::FromStr;

named!(int <&str, i32>,
       map_res!(recognize!(digit1), FromStr::from_str)
);

fn make_parser<'a>(validate: bool) -> impl FnMut(&'a str) -> bool {
    let byr = tuple((
        tag("byr:"),
        cond(validate, verify(int, |n| 1920 <= *n && *n <= 2002)),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let iyr = tuple((
        tag("iyr:"),
        cond(validate, verify(int, |n| 2010 <= *n && *n <= 2020)),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let eyr = tuple((
        tag("eyr:"),
        cond(validate, verify(int, |n| 2020 <= *n && *n <= 2030)),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let hgt = tuple((
        tag("hgt:"),
        cond(
            validate,
            verify(pair(int, alt((tag("cm"), tag("in")))), |(h, u)| match *u {
                "cm" => 150 <= *h && *h <= 193,
                "in" => 59 <= *h && *h <= 76,
                _ => unreachable!(),
            }),
        ),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let hcl = tuple((
        tag("hcl:"),
        cond(
            validate,
            pair(
                tag("#"),
                take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit()),
            ),
        ),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let ecl = tuple((
        tag("ecl:"),
        cond(
            validate,
            alt((
                tag("amb"),
                tag("blu"),
                tag("brn"),
                tag("gry"),
                tag("grn"),
                tag("hzl"),
                tag("oth"),
            )),
        ),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let pid = tuple((
        tag("pid:"),
        cond(validate, take_while_m_n(9, 9, |c: char| c.is_ascii_digit())),
        cond(!validate, take_till(|c: char| c.is_ascii_whitespace())),
    ));
    let cid = pair(tag("cid:"), take_till(|c: char| c.is_ascii_whitespace()));
    let mut parser = permutation((
        preceded(multispace0, byr),
        preceded(multispace0, iyr),
        preceded(multispace0, eyr),
        preceded(multispace0, hgt),
        preceded(multispace0, hcl),
        preceded(multispace0, ecl),
        preceded(multispace0, pid),
        opt(preceded(multispace0, cid)),
    ));
    move |s| parser(s).is_ok()
}

fn count_matches<'a, F: FnMut(&'a str) -> bool>(mut parser: F, input: &'a str) -> usize {
    input.split("\n\n").filter(|line| parser(line)).count()
}

pub fn part1(input: &str) -> usize {
    count_matches(make_parser(false), input)
}

pub fn part2(input: &str) -> usize {
    count_matches(make_parser(true), input)
}
