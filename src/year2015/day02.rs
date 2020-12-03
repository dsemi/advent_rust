fn process(input: &str, f: fn(i64, i64, i64) -> i64) -> i64 {
    input
        .lines()
        .map(|line| {
            let v: Vec<i64> = line.split("x").map(|x| x.parse().unwrap()).collect();
            f(v[0], v[1], v[2])
        })
        .sum()
}

pub fn part1(input: &str) -> i64 {
    process(input, |l, w, h| {
        2 * l * w + 2 * w * h + 2 * h * l + [l * w, l * h, w * h].iter().min().unwrap()
    })
}

pub fn part2(input: &str) -> i64 {
    process(input, |l, w, h| {
        l * w * h + 2 * [l + w, l + h, w + h].iter().min().unwrap()
    })
}
