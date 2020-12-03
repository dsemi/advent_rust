use fancy_regex::Regex;

pub fn part1(input: &str) -> usize {
    let re = Regex::new(r"(.)\1").unwrap();
    input
        .lines()
        .filter(|&line| {
            line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
                && re.is_match(line).unwrap()
                && !["ab", "cd", "pq", "xy"].iter().any(|x| line.contains(x))
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let re = Regex::new(r"(.)(.).*\1\2").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();
    input
        .lines()
        .filter(|&line| re.is_match(line).unwrap() && re2.is_match(line).unwrap())
        .count()
}
