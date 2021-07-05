use regex::Regex;

fn count_matches(regexes: Vec<Regex>, input: &str) -> usize {
    input.split("\n\n")
        .filter(|line| regexes.iter().all(|r| r.is_match(line)))
        .count()
}

pub fn part1(input: &str) -> usize {
    let regexes = vec![
        Regex::new(r"(^|\s)byr:").unwrap(),
        Regex::new(r"(^|\s)iyr:").unwrap(),
        Regex::new(r"(^|\s)eyr:").unwrap(),
        Regex::new(r"(^|\s)hgt:").unwrap(),
        Regex::new(r"(^|\s)hcl:").unwrap(),
        Regex::new(r"(^|\s)ecl:").unwrap(),
        Regex::new(r"(^|\s)pid:").unwrap(),
    ];
    count_matches(regexes, input)
}

pub fn part2(input: &str) -> usize {
    let regexes = vec![
        Regex::new(r"(^|\s)byr:(19[2-9][0-9]|200[0-2])(\s|$)").unwrap(),
        Regex::new(r"(^|\s)iyr:(201[0-9]|2020)(\s|$)").unwrap(),
        Regex::new(r"(^|\s)eyr:(202[0-9]|2030)(\s|$)").unwrap(),
        Regex::new(r"(^|\s)hgt:(1[5-8][0-9]|19[0-3])cm|hgt:(59|6[0-9]|7[0-6])in(\s|$)").unwrap(),
        Regex::new(r"(^|\s)hcl:#[0-9a-f]{6}(\s|$)").unwrap(),
        Regex::new(r"(^|\s)ecl:(amb|blu|brn|gry|grn|hzl|oth)(\s|$)").unwrap(),
        Regex::new(r"(^|\s)pid:[0-9]{9}(\s|$)").unwrap(),
    ];
    count_matches(regexes, input)
}
