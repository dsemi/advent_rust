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
    let (mut i, mut idx_of0, mut val_aft0) = (0, 0, 0);
    for v in 1..=50_000_000 {
        i = (i + step) % v + 1;
        if i <= idx_of0 {
            idx_of0 += 1;
        } else if i == idx_of0 + 1 {
            val_aft0 = v;
        }
    }
    val_aft0
}
