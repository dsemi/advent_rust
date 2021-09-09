use regex::Regex;
use std::cmp::max;

fn partitions<F: FnMut(&[i32])>(n: usize, t: i32, f: &mut F) {
    fn fun<F: FnMut(&[i32])>(n: usize, t: i32, buf: &mut [i32], f: &mut F) {
        if n == 0 {
            buf[n] = t;
            f(buf);
        } else {
            for x in 0..=t {
                buf[n] = x;
                fun(n - 1, t - x, buf, f);
            }
        }
    }
    let mut buf = vec![0; n];
    fun(n - 1, t, &mut buf, f);
}

fn parse_ingredients(s: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"-?\d+").unwrap();
    s.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|n| n.as_str().parse().unwrap())
                .collect()
        })
        .collect()
}

fn max_score(total: i32, cal_pred: fn(i32) -> bool, ings: Vec<Vec<i32>>) -> i32 {
    let mut res = i32::MIN;
    partitions(ings.len(), total, &mut |ms| {
        let mut v = vec![0; 5];
        for i in 0..ms.len() {
            for (j, e) in v.iter_mut().enumerate() {
                *e += ms[i] * ings[i][j];
            }
        }
        if cal_pred(v[4]) {
            res = max(res, v[..4].iter().map(|&x| max(0, x)).product());
        }
    });
    res
}

pub fn part1(input: &str) -> i32 {
    max_score(100, |_| true, parse_ingredients(input))
}

pub fn part2(input: &str) -> i32 {
    max_score(100, |x| x == 500, parse_ingredients(input))
}
