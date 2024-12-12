use hashbrown::HashMap;

type Map = HashMap<(char, char, char, char, char), char>;

fn parse(input: &str) -> (Map, Vec<(i64, char)>) {
    let (initial, rest) = input.split_once("\n\n").unwrap();
    (
        rest.lines()
            .map(|line| {
                let chs = line.chars().collect::<Vec<_>>();
                ((chs[0], chs[1], chs[2], chs[3], chs[4]), chs[chs.len() - 1])
            })
            .collect(),
        initial
            .replace("initial state: ", "")
            .chars()
            .enumerate()
            .map(|(i, c)| (i as i64, c))
            .collect(),
    )
}

fn next_gen(m: &Map, s: Vec<(i64, char)>) -> Vec<(i64, char)> {
    let (x0, x1) = (s[0].0, s[s.len() - 1].0);
    let mut v = vec![(x0 - 3, '.'), (x0 - 2, '.'), (x0 - 1, '.')];
    v.extend(s);
    for i in 1..=3 {
        v.push((x1 + i, '.'));
    }
    v[..]
        .windows(5)
        .map(|x| (x[2].0, m[&(x[0].1, x[1].1, x[2].1, x[3].1, x[4].1)]))
        .collect()
}

fn sum_indices(s: &[(i64, char)]) -> i64 {
    s.iter().filter_map(|(i, c)| (*c == '#').then_some(i)).sum()
}

pub fn part1(input: &str) -> i64 {
    let (m, mut start) = parse(input);
    for _ in 0..20 {
        start = next_gen(&m, start);
    }
    sum_indices(&start)
}

fn find_arith(m: &Map, mut x: Vec<(i64, char)>) -> (i64, i64, i64) {
    let mut prev = 0;
    for c in 0.. {
        let si = sum_indices(&x);
        x = next_gen(m, x);
        let prev2 = sum_indices(&x) - si;
        if prev2 == prev {
            return (c, prev, si);
        }
        prev = prev2;
    }
    panic!("No solution found")
}

pub fn part2(input: &str) -> i64 {
    let (m, start) = parse(input);
    let (n, diff, tot) = find_arith(&m, start);
    (50_000_000_000 - n) * diff + tot
}
