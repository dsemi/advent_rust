use itertools::Itertools;

fn parse_nums(s: &str) -> Vec<i64> {
    s.lines().map(|x| x.parse().unwrap()).collect()
}

fn find_first_invalid(ns: &Vec<i64>) -> i64 {
    let mut n = 25;
    loop {
        if !ns[n - 25..n]
            .iter()
            .combinations(2)
            .any(|combo| combo.into_iter().sum::<i64>() == ns[n])
        {
            return ns[n];
        }
        n += 1;
    }
}

pub fn part1(input: &str) -> i64 {
    find_first_invalid(&parse_nums(input))
}

pub fn part2(input: &str) -> Option<i64> {
    let ns = parse_nums(input);
    let n = find_first_invalid(&ns);
    let (mut lo, mut hi, mut acc) = (0, 0, 0);
    while acc != n {
        if acc < n {
            acc += ns[hi];
            hi += 1;
        } else if acc > n {
            acc -= ns[lo];
            lo += 1;
        }
    }
    let arr = &ns[lo..hi];
    Some(arr.iter().min()? + arr.iter().max()?)
}
