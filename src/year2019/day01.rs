use std::num::ParseIntError;

pub fn part1(input: &str) -> Result<i64, ParseIntError> {
    let mut total = 0;
    for line in input.lines() {
        total += line.parse::<i64>()? / 3 - 2;
    }
    Ok(total)
}

pub fn part2(input: &str) -> Result<i64, ParseIntError> {
    let mut total = 0;
    for line in input.lines() {
        let mut fuel = line.parse::<i64>()? / 3 - 2;
        while fuel > 0 {
            total += fuel;
            fuel = fuel / 3 - 2;
        }
    }
    Ok(total)
}
