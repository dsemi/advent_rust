use ahash::AHashMap;
use itertools::Itertools;

pub fn part1(input: &str) -> i32 {
    let cnts = input
        .chars()
        .chunks(150)
        .into_iter()
        .map(|x| {
            let mut t = AHashMap::new();
            for c in x {
                *t.entry(c).or_insert(0) += 1;
            }
            t
        })
        .min_by_key(|x| x[&'0'])
        .unwrap();
    cnts[&'1'] * cnts[&'2']
}

pub fn part2(input: &str) -> String {
    let mut pts = vec!['2'; 150];
    for chunk in input.chars().chunks(150).into_iter() {
        for (i, c) in chunk.enumerate() {
            if pts[i] == '2' {
                pts[i] = c;
            }
        }
    }
    std::iter::once("\n".to_owned())
        .chain(itertools::Itertools::intersperse(
            pts.into_iter()
                .map(|x| if x == '0' { ' ' } else { '#' })
                .chunks(25)
                .into_iter()
                .map(|x| x.collect::<String>()),
            "\n".to_string(),
        ))
        .collect()
}
