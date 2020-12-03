use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn parse_happiness(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut d = HashMap::new();
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)").unwrap();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let (p1, op, n, p2) = (&cap[1], if &cap[2] == "gain" { 1 } else { -1 },
                               cap[3].parse::<i32>().unwrap(), &cap[4]);
        d.entry(p1.to_string()).or_insert_with(HashMap::new).insert(p2.to_string(), op * n);
    }
    d
}

fn max_happiness(d: HashMap<String, HashMap<String, i32>>) -> Option<i32> {
    d.keys()
        .permutations(d.len())
        .map(|mut perm| {
            perm.push(perm[0]);
            perm.windows(2).map(|p| d.get(p[0]).unwrap().get(p[1]).unwrap_or(&0)
                             + d.get(p[1]).unwrap().get(p[0]).unwrap_or(&0)).sum::<i32>()})
        .max()
}

pub fn part1(input: &str) -> Option<i32> {
    max_happiness(parse_happiness(input))
}

pub fn part2(input: &str) -> Option<i32> {
    let mut d = parse_happiness(input);
    d.insert("me".to_string(), HashMap::new());
    max_happiness(d)
}
