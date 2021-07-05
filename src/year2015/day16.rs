use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref TAPE: HashMap<&'static str, fn(i32) -> bool> = {
        let mut m: HashMap<&'static str, fn(i32) -> bool> = HashMap::new();
        m.insert("children", |x| x == 3);
        m.insert("cats", |x| x == 7);
        m.insert("samoyeds", |x| x == 2);
        m.insert("pomeranians", |x| x == 3);
        m.insert("akitas", |x| x == 0);
        m.insert("vizslas", |x| x == 0);
        m.insert("goldfish", |x| x == 5);
        m.insert("trees", |x| x == 3);
        m.insert("cars", |x| x == 2);
        m.insert("perfumes", |x| x == 1);
        m
    };
}

fn solve<'a>(input: &str, tape: HashMap<&'a str, fn(i32) -> bool>) -> Option<usize> {
    let re = Regex::new(r"(\w+): (\d+)").unwrap();
    input
        .lines()
        .position(|line| {
            re.captures_iter(line).all(|cap| {
                let key = &cap[1];
                let val = cap[2].parse().unwrap();
                tape.get(key).unwrap_or_else(|| &TAPE[key])(val)
            })
        })
        .map(|x| x + 1)
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, HashMap::new())
}

pub fn part2(input: &str) -> Option<usize> {
    solve(
        input,
        {
            let mut m: HashMap<&'static str, fn(i32) -> bool> = HashMap::new();
            m.insert("cats", |x| x > 7);
            m.insert("pomeranians", |x| x < 3);
            m.insert("goldfish", |x| x < 5);
            m.insert("trees", |x| x > 3);
            m
        },
    )
}
