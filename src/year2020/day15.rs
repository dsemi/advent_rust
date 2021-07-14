use std::collections::HashMap;

fn solve(n: u32, input: &str) -> u32 {
    let bdy = n / 10;
    let mut m = vec![0; bdy as usize];
    let mut m2: HashMap<u32, u32> = HashMap::new();
    let mut j = 1;
    for v in input.split(",") {
        m[v.parse::<usize>().unwrap()] = j;
        j += 1;
    }
    let mut result = 0;
    for i in j..n {
        if result < bdy {
            result = std::mem::replace(&mut m[result as usize], i);
            if result != 0 {
                result = i - result;
            }
        } else {
            result = match m2.insert(result, i) {
                Some(v) => i - v,
                None => 0,
            }
        }
    }
    result
}

pub fn part1(input: &str) -> u32 {
    solve(2020, input)
}

pub fn part2(input: &str) -> u32 {
    solve(30_000_000, input)
}
