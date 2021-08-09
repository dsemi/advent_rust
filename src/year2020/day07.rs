use ahash::AHashMap;
use regex::Regex;

fn parse_bags(s: &str) -> AHashMap<String, Vec<(i64, String)>> {
    let mut m = AHashMap::new();
    let re = Regex::new(r"(?:(\d+) )?(\w+ \w+) bags?").unwrap();
    for line in s.lines() {
        let bags = re.captures_iter(line).collect::<Vec<_>>();
        m.insert(
            bags[0][2].to_string(),
            bags[1..]
                .iter()
                .filter(|x| &x[2] != "no other")
                .map(|x| (x[1].parse().unwrap(), x[2].to_string()))
                .collect(),
        );
    }
    m
}

fn holds_shiny_gold(m: &AHashMap<String, Vec<(i64, String)>>, k: &str) -> bool {
    m[k]
        .iter()
        .any(|(_, k2)| k2 == "shiny gold" || holds_shiny_gold(m, &k2))
}

pub fn part1(input: &str) -> usize {
    let m = parse_bags(input);
    m.keys().filter(|&k| holds_shiny_gold(&m, k)).count()
}

fn count_bags(m: &AHashMap<String, Vec<(i64, String)>>, k: &str) -> i64 {
    m[k]
        .iter()
        .map(|(n, k2)| n + n * count_bags(m, &k2))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let m = parse_bags(input);
    count_bags(&m, "shiny gold")
}
