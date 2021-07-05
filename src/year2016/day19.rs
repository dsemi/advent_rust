use std::cmp::max;

pub fn part1(input: &str) -> i64 {
    let n: i64 = input.parse().unwrap();
    1 + 2 * (n - 2_i64.pow((n as f64).log2() as u32))
}

pub fn part2(input: &str) -> i64 {
    let n: i64 = input.parse().unwrap();
    let p3 = 3_i64.pow((n as f64).log(3.0) as u32);
    let ans = n - p3;
    let ans2 = ans + max(0, ans - p3);
    if ans2 == 0 {
        p3
    } else {
        ans
    }
}
