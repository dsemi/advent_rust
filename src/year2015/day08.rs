fn encoded_len(s: &str) -> usize {
    let mut i = s.chars();
    let mut len = 0;
    while let Some(c) = i.next() {
        match c {
            '\\' => {
                if Some('x') == i.next() {
                    i.next();
                    i.next();
                }
                len += 1;
            }
            '"' => (),
            _ => len += 1,
        }
    }
    len
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.len() - encoded_len(line))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().filter(|&c| c == '\\' || c == '"').count() + 2)
        .sum()
}
