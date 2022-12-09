use scan_fmt::scan_fmt as scanf;

fn move_stacks(input: &str, in_order: bool) -> String {
    let (x, y) = input.split_once("\n\n").unwrap();
    let mut crates = x.lines().collect::<Vec<_>>();
    crates.reverse();
    let mut stacks = vec![vec![]; (crates[0].len() + 1) / 4];
    for line in crates {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if ('A'..='Z').contains(&c) {
                stacks[i].push(c);
            }
        }
    }
    for line in y.lines() {
        let (n, a, b) = scanf!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        let mut tmp = (0..n)
            .map(|_| stacks[a - 1].pop().unwrap())
            .collect::<Vec<_>>();
        if in_order {
            tmp.reverse();
        }
        stacks[b - 1].extend(tmp);
    }
    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

pub fn part1(input: &str) -> String {
    move_stacks(input, false)
}

pub fn part2(input: &str) -> String {
    move_stacks(input, true)
}
