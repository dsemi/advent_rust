use ahash::AHashSet;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while, take_while_m_n};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map_res, recognize, verify};
use nom::sequence::{pair, terminated};
use nom::IResult;
use std::str::FromStr;

fn int(i: &str) -> IResult<&str, i32> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn parse(mut inp: &str, validate: bool) -> IResult<&str, ()> {
    let mut req_fields: AHashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    while !req_fields.is_empty() {
        let (i, field) = terminated(take_while(|c: char| c.is_ascii_alphabetic()), tag(":"))(inp)?;
        let i = if validate {
            match field {
                "byr" => verify(int, |n| 1920 <= *n && *n <= 2002)(i)?.0,
                "iyr" => verify(int, |n| 2010 <= *n && *n <= 2020)(i)?.0,
                "eyr" => verify(int, |n| 2020 <= *n && *n <= 2030)(i)?.0,
                "hgt" => {
                    verify(pair(int, alt((tag("cm"), tag("in")))), |(h, u)| match *u {
                        "cm" => 150 <= *h && *h <= 193,
                        "in" => 59 <= *h && *h <= 76,
                        _ => unreachable!(),
                    })(i)?
                    .0
                }
                "hcl" => {
                    let i = tag("#")(i)?.0;
                    take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit())(i)?.0
                }
                "ecl" => {
                    alt((
                        tag("amb"),
                        tag("blu"),
                        tag("brn"),
                        tag("gry"),
                        tag("grn"),
                        tag("hzl"),
                        tag("oth"),
                    ))(i)?
                    .0
                }
                "pid" => take_while_m_n(9, 9, |c: char| c.is_ascii_digit())(i)?.0,
                _ => take_till(|c: char| c.is_ascii_whitespace())(i)?.0,
            }
        } else {
            take_till(|c: char| c.is_ascii_whitespace())(i)?.0
        };
        inp = multispace0(i)?.0;
        req_fields.remove(&field);
    }
    Ok((inp, ()))
}

fn count_matches(input: &str, validate: bool) -> usize {
    input
        .split("\n\n")
        .filter(|line| matches!(parse(line, validate), Ok(_)))
        .count()
}

pub fn part1(input: &str) -> usize {
    count_matches(input, false)
}

pub fn part2(input: &str) -> usize {
    count_matches(input, true)
}
