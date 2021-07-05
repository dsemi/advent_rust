fn consecutive_runs(s: &str) -> Vec<i32> {
    let b = s.as_bytes();
    let mut c = 1;
    let mut result = Vec::new();
    for i in 0..b.len() - 1 {
        if b[i] == b[i + 1] {
            c += 1;
        } else {
            result.push(c);
            c = 1;
        }
    }
    result.push(c);
    result
}

fn num_valid(input: &str, f: fn(&str) -> bool) -> usize {
    let pts: Vec<usize> = input.split('-').map(|x| x.parse().unwrap()).collect();
    (pts[0]..pts[1] + 1)
        .filter(|v| {
            let s = v.to_string();
            s.chars().zip(s.chars().skip(1)).all(|(a, b)| a <= b) && f(&s)
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    num_valid(input, |s| consecutive_runs(s).into_iter().any(|x| x >= 2))
}

pub fn part2(input: &str) -> usize {
    num_valid(input, |s| consecutive_runs(s).into_iter().any(|x| x == 2))
}
