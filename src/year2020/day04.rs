use crate::utils::parsers::*;
use hashbrown::HashSet;

fn validate_val<'a>(field: &'a str) -> impl Parser<&'a str, (), ContextError> {
    move |i: &mut &'a str| {
        match field {
            "byr" => {
                i32.verify(|n| (1920..=2002).contains(n)).parse_next(i)?;
            }
            "iyr" => {
                i32.verify(|n| (2010..=2020).contains(n)).parse_next(i)?;
            }
            "eyr" => {
                i32.verify(|n| (2020..=2030).contains(n)).parse_next(i)?;
            }
            "hgt" => {
                alt((
                    (i32.verify(|h| (150..=193).contains(h)), "cm"),
                    (i32.verify(|h| (59..=76).contains(h)), "in"),
                ))
                .parse_next(i)?;
            }
            "hcl" => {
                preceded('#', hex_digit1.verify(|h: &str| h.len() == 6)).parse_next(i)?;
            }
            "ecl" => {
                alt(("amb", "blu", "brn", "gry", "grn", "hzl", "oth")).parse_next(i)?;
            }
            "pid" => {
                digit1.verify(|h: &str| h.len() == 9).parse_next(i)?;
            }
            _ => {
                rest.value(()).parse_next(i)?;
            }
        };
        Ok(())
    }
}

fn parse<'a>(validate: bool) -> impl Parser<&'a str, (), ContextError> {
    move |i: &mut &'a str| {
        let mut req_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();
        while !req_fields.is_empty() {
            let (field, mut val) =
                separated_pair(alpha1, ':', take_till(0.., char::is_whitespace)).parse_next(i)?;
            if validate {
                validate_val(field).parse_next(&mut val)?;
            }
            multispace0.parse_next(i)?;
            req_fields.remove(&field);
        }
        rest.parse_next(i)?;
        Ok(())
    }
}

fn count_matches(input: &str, validate: bool) -> usize {
    input
        .split("\n\n")
        .filter(|line| parse(validate).parse(line).is_ok())
        .count()
}

pub fn part1(input: &str) -> usize {
    count_matches(input, false)
}

pub fn part2(input: &str) -> usize {
    count_matches(input, true)
}
