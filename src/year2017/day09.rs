fn process(input: &str) -> (usize, usize) {
    let mut score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut garbage_count = 0;
    let mut ignore_next = false;
    for x in input.chars() {
        if ignore_next {
            ignore_next = false;
        } else if in_garbage {
            match x {
                '!' => ignore_next = true,
                '>' => in_garbage = false,
                _ => garbage_count += 1,
            }
        } else if x == '}' {
            score += depth;
            depth -= 1;
        } else if x == '{' {
            depth += 1;
        } else if x == '<' {
            in_garbage = true;
        }
    }
    (score, garbage_count)
}

pub fn part1(input: &str) -> usize {
    process(input).0
}

pub fn part2(input: &str) -> usize {
    process(input).1
}
