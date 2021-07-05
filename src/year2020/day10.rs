use std::collections::HashMap;

fn parse_nums(s: &str) -> Vec<i64> {
    let mut ns: Vec<i64> = s.lines().map(|x| x.parse().unwrap()).collect();
    ns.sort();
    ns.insert(0, 0);
    ns.push(ns.last().unwrap() + 3);
    ns
}

pub fn part1(input: &str) -> i64 {
    let ns = parse_nums(input);
    let mut cnt = HashMap::new();
    for (a, b) in ns.iter().zip(ns[1..].iter()) {
        let counter = cnt.entry(b - a).or_insert(0);
        *counter += 1;
    }
    cnt[&1] * cnt[&3]
}

pub fn part2(input: &str) -> i64 {
    let ns = parse_nums(input);
    let mut dp = vec![0; ns[ns.len()-1] as usize + 1];
    dp[0] = 1;
    for n in ns[1..].iter() {
        for i in (n-3) .. *n {
            if i >= 0 {
                dp[*n as usize] += dp[i as usize];
            }
        }
    }
    dp[ns[ns.len()-1] as usize]
}
