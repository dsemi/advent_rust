fn move_up(result: &mut Vec<i64>, fstree: &mut Vec<i64>) {
    let size = fstree.pop().unwrap();
    *fstree.last_mut().unwrap() += size;
    result.push(size);
}

fn all_sizes(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    let mut fstree = vec![0];
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            if line.ends_with('/') {
                while fstree.len() > 1 {
                    move_up(&mut result, &mut fstree);
                }
            } else if line.ends_with("..") {
                move_up(&mut result, &mut fstree);
            } else {
                fstree.push(0);
            }
        } else if line.as_bytes()[0].is_ascii_digit() {
            *fstree.last_mut().unwrap() += line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
    while fstree.len() > 1 {
        move_up(&mut result, &mut fstree);
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
