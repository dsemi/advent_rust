fn calc_steps(mut ns: Vec<i64>, f: fn(i64) -> i64) -> usize {
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

pub fn part1(input: Vec<i64>) -> usize {
    calc_steps(input, |x| x + 1)
}

pub fn part2(input: Vec<i64>) -> usize {
    calc_steps(input, |x| if x >= 3 { x - 1 } else { x + 1 })
}
