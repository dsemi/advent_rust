use ahash::AHashMap;

fn parse_bags(s: &str) -> AHashMap<&str, Vec<(u32, &str)>> {
    s.lines()
        .map(|line| {
            let (outer_bag, inner_bags) = line[..line.len() - 1].split_once(" contain ").unwrap();
            (
                outer_bag.rsplit_once(' ').unwrap().0,
                inner_bags
                    .split(", ")
                    .filter(|&bag| bag != "no other bags")
                    .map(|bag| {
                        let (n, name) = bag.rsplit_once(' ').unwrap().0.split_once(' ').unwrap();
                        (n.parse().unwrap(), name)
                    })
                    .collect(),
            )
        })
        .collect()
}

fn holds_shiny_gold<'a>(
    cache: &mut AHashMap<&'a str, bool>,
    m: &AHashMap<&str, Vec<(u32, &'a str)>>,
    k: &'a str,
) -> bool {
    match cache.get(k) {
        None => {
            let v = m[k]
                .iter()
                .any(|(_, k2)| k2 == &"shiny gold" || holds_shiny_gold(cache, m, k2));
            cache.insert(k, v);
            v
        }
        Some(v) => *v,
    }
}

pub fn part1(input: &str) -> usize {
    let m = parse_bags(input);
    let mut c = AHashMap::new();
    m.keys()
        .filter(|&k| holds_shiny_gold(&mut c, &m, k))
        .count()
}

fn count_bags(m: &AHashMap<&str, Vec<(u32, &str)>>, k: &str) -> u32 {
    m[k].iter().map(|(n, k2)| n + n * count_bags(m, k2)).sum()
}

pub fn part2(input: &str) -> u32 {
    count_bags(&parse_bags(input), "shiny gold")
}
