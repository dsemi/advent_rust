fn all_sizes(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let mut fstree = Vec::new();
    let mut size = 0;
    for line in input.lines().skip(1) {
        if line.starts_with("$ cd ") {
            if line.ends_with("..") {
                result.push(size);
                size += fstree.pop().unwrap();
            } else {
                fstree.push(std::mem::take(&mut size));
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            size += line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
    result.push(size);
    result.extend(fstree.into_iter().rev().scan(size, |acc, x| {
        *acc += x;
        Some(*acc)
    }));
    result
}

pub fn part1(input: &str) -> i64 {
    all_sizes(input).into_iter().filter(|&s| s <= 100000).sum()
}

pub fn part2(input: &str) -> Option<i64> {
    let sizes = all_sizes(input);
    let target = *sizes.last().unwrap() - 40000000;
    sizes.into_iter().filter(|&s| s >= target).min()
}
