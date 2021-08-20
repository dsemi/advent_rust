use ahash::AHashMap;

lazy_static! {
    static ref TAPE: AHashMap<&'static str, fn(i32) -> bool> = {
        let mut m: AHashMap<&'static str, fn(i32) -> bool> = AHashMap::new();
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

fn solve(input: &str, tape: AHashMap<&str, fn(i32) -> bool>) -> Option<usize> {
    input
        .lines()
        .position(|line| {
            line.split_once(": ").unwrap().1.split(", ").all(|attr| {
                let (key, val) = attr.split_once(": ").unwrap();
                tape.get(key).unwrap_or_else(|| &TAPE[key])(val.parse().unwrap())
            })
        })
        .map(|x| x + 1)
}

pub fn part1(input: &str) -> Option<usize> {
    solve(input, AHashMap::new())
}

pub fn part2(input: &str) -> Option<usize> {
    solve(
        input,
        {
            let mut m: AHashMap<&'static str, fn(i32) -> bool> = AHashMap::new();
            m.insert("cats", |x| x > 7);
            m.insert("pomeranians", |x| x < 3);
            m.insert("goldfish", |x| x < 5);
            m.insert("trees", |x| x > 3);
            m
        },
    )
}
