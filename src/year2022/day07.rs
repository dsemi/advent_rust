fn all_sizes(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let mut fstree = vec![0];
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            if line.ends_with("/") {
                while fstree.len() > 1 {
                    let size = fstree.pop().unwrap();
                    *fstree.last_mut().unwrap() += size;
                    result.push(size);
                }
            } else if line.ends_with("..") {
                let size = fstree.pop().unwrap();
                *fstree.last_mut().unwrap() += size;
                result.push(size);
            } else {
                fstree.push(0);
            }
        } else if (b'0'..=b'9').contains(&line.as_bytes()[0]) {
            *fstree.last_mut().unwrap() += line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
    while fstree.len() > 1 {
        let size = fstree.pop().unwrap();
        *fstree.last_mut().unwrap() += size;
        result.push(size);
    }
    result.push(fstree.pop().unwrap());
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
