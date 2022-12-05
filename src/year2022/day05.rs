use scan_fmt::scan_fmt as scanf;

fn move_stacks(input: &str, in_order: bool) -> String {
    let (x, y) = input.split_once("\n\n").unwrap();
    let mut crates = x.lines().collect::<Vec<_>>();
    crates.reverse();
    let mut stacks = vec![vec![]; (crates[0].len() + 1) / 4];
    for line in crates {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c >= 'A' && c <= 'Z' {
                stacks[i].push(c);
            }
        }
    }
    for line in y.lines() {
        let (n, a, b) = scanf!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        if in_order {
            let seg = stacks[a - 1][stacks[a - 1].len() - n..].to_vec();
            let len = stacks[a - 1].len();
            stacks[a - 1].truncate(len - n);
            stacks[b - 1].extend(seg);
        } else {
            for _ in 0..n {
                let v = stacks[a - 1].pop().unwrap();
                stacks[b - 1].push(v);
            }
        }
    }
    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

pub fn part1(input: &str) -> String {
    move_stacks(input, false)
}

pub fn part2(input: &str) -> String {
    move_stacks(input, true)
}
