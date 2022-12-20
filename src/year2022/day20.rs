fn mix(input: &str, scale: i64, times: usize) -> i64 {
    let ns: Vec<i64> = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap() * scale)
        .collect();
    let mut locs: Vec<usize> = (0..ns.len()).collect();
    let m = locs.len() as i64;
    for _ in 0..times {
        for (i, n) in ns.iter().enumerate() {
            let loc = locs.iter().position(|&l| l == i).unwrap();
            locs.remove(loc);
            locs.insert((n + loc as i64).rem_euclid(m - 1) as usize, i);
        }
    }
    let idx = ns.iter().position(|&n| n == 0).unwrap();
    let z = locs.iter().position(|&l| l == idx).unwrap() as i64;
    ns[locs[(z + 1000).rem_euclid(m) as usize]]
        + ns[locs[(z + 2000).rem_euclid(m) as usize]]
        + ns[locs[(z + 3000).rem_euclid(m) as usize]]
}

pub fn part1(input: &str) -> i64 {
    mix(input, 1, 1)
}

pub fn part2(input: &str) -> i64 {
    mix(input, 811589153, 10)
}
