fn analyze(line: &str) -> Result<usize, u64> {
    let mut stack = vec![];
    for c in line.chars() {
        if let Some(i) = "([{<".chars().position(|x| x == c) {
            stack.push(")]}>".chars().nth(i).unwrap());
        } else if stack.pop().unwrap() != c {
            return Err(match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                x => panic!("Bad char: {}", x),
            });
        }
    }
    Ok(stack
        .into_iter()
        .rev()
        .map(|c| ")]}>".chars().position(|x| x == c).unwrap() + 1)
        .fold(0, |a, b| a * 5 + b))
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(analyze).filter_map(|r| r.err()).sum()
}

pub fn part2(input: &str) -> usize {
    let mut ns = input
        .lines()
        .map(analyze)
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
    ns.sort_unstable();
    ns[ns.len() / 2]
}
