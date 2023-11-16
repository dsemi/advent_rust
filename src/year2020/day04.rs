use crate::utils::parsers::*;
use ahash::AHashSet;

fn validate_val(field: &str) -> impl FnMut(&str) -> IResult<&str, ()> + '_ {
    move |i| {
        let i = match field {
            "byr" => verify(i32, |n| (1920..=2002).contains(n))(i)?.0,
            "iyr" => verify(i32, |n| (2010..=2020).contains(n))(i)?.0,
            "eyr" => verify(i32, |n| (2020..=2030).contains(n))(i)?.0,
            "hgt" => {
                alt((
                    pair(verify(i32, |h| (150..=193).contains(h)), tag("cm")),
                    pair(verify(i32, |h| (59..=76).contains(h)), tag("in")),
                ))(i)?
                .0
            }
            "hcl" => preceded(tag("#"), verify(hex_digit1, |h: &str| h.len() == 6))(i)?.0,
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
            "pid" => verify(digit1, |h: &str| h.len() == 9)(i)?.0,
            _ => rest(i)?.0,
        };
        Ok((i, ()))
    }
}

fn parse(mut inp: &str, validate: bool) -> IResult<&str, ()> {
    let mut req_fields: AHashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    while !req_fields.is_empty() {
        let (i, (field, val)) =
            separated_pair(alpha1, tag(":"), take_till(char::is_whitespace))(inp)?;
        if validate {
            all_consuming(validate_val(field))(val)?;
        }
        inp = multispace0(i)?.0;
        req_fields.remove(&field);
    }
    Ok((inp, ()))
}

fn count_matches(input: &str, validate: bool) -> usize {
    input
        .split("\n\n")
        .filter(|line| parse(line, validate).is_ok())
        .count()
}

pub fn part1(input: &str) -> usize {
    count_matches(input, false)
}

pub fn part2(input: &str) -> usize {
    count_matches(input, true)
}
