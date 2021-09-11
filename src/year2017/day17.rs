pub fn part1(input: &str) -> usize {
    let step: usize = input.parse().unwrap();
    let mut list = vec![0];
    let mut idx = 0;
    for v in 1..=2017 {
        idx = (idx + step) % v + 1;
        list.insert(idx, v);
    }
    list[idx + 1]
}

pub fn part2(input: &str) -> usize {
    let step: usize = input.parse().unwrap();
    let (mut pos, mut n, mut val_aft0) = (0, 0, 0);
    while n < 50_000_000 {
        if pos == 1 {
            val_aft0 = n;
        }
        let skip = (n - pos) / step + 1;
        n += skip;
        pos = (pos + skip * (step + 1) - 1) % n + 1;
    }
    val_aft0
}
