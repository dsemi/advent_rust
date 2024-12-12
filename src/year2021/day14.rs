use hashbrown::HashMap;

fn polymerize(input: &str, n: usize) -> usize {
    let (tmpl, rest) = input.split_once("\n\n").unwrap();
    let vtmpl = tmpl.chars().collect::<Vec<_>>();
    let d: HashMap<Vec<char>, char> = rest
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.chars().collect(), b.chars().next().unwrap()))
        .collect();
    let mut cnts = HashMap::new();
    for k in vtmpl.windows(2) {
        *cnts.entry(k.to_owned()).or_insert(0) += 1;
    }
    for _ in 0..n {
        let mut cnts2 = HashMap::new();
        for (k, v) in cnts {
            let rep = d[&k];
            *cnts2.entry(vec![k[0], rep]).or_insert(0) += v;
            *cnts2.entry(vec![rep, k[1]]).or_insert(0) += v;
        }
        cnts = cnts2;
    }
    let mut lets = HashMap::new();
    cnts.into_iter()
        .for_each(|(k, v)| *lets.entry(k[0]).or_insert(0) += v);
    *lets.entry(*vtmpl.last().unwrap()).or_insert(0) += 1;
    lets.values().max().unwrap() - lets.values().min().unwrap()
}

pub fn part1(input: &str) -> usize {
    polymerize(input, 10)
}

pub fn part2(input: &str) -> usize {
    polymerize(input, 40)
}
