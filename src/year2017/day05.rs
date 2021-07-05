fn calc_steps(input: &str, f: fn(i64) -> i64) -> usize {
    let mut ns: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut i = 0;
    let mut res = 0;
    while i >= 0 && i < ns.len() as i64 {
        let val = ns[i as usize];
        ns[i as usize] = f(val);
        i += val;
        res += 1;
    }
    res
}

pub fn part1(input: &str) -> usize {
    calc_steps(input, |x| x + 1)
}

pub fn part2(input: &str) -> usize {
    calc_steps(input, |x| if x >= 3 { x - 1 } else { x + 1 })
}
