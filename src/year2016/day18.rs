fn safe_or_trap(a: char, b: char, c: char) -> char {
    match (a, b, c) {
        ('^', '^', '.') => '^',
        ('.', '^', '^') => '^',
        ('^', '.', '.') => '^',
        ('.', '.', '^') => '^',
        _ => '.',
    }
}

fn num_safe(n: usize, input: &str) -> usize {
    let mut state = input.chars().collect::<Vec<_>>();
    state.push('.');
    let mut total = 0;
    for _ in 0..n {
        total += state[..state.len() - 1]
            .iter()
            .filter(|&x| *x == '.')
            .count();
        let mut prev = '.';
        for i in 0..state.len() - 1 {
            state[i] = safe_or_trap(
                std::mem::replace(&mut prev, state[i]),
                state[i],
                state[i + 1],
            );
        }
    }
    total
}

pub fn part1(input: &str) -> usize {
    num_safe(40, input)
}

pub fn part2(input: &str) -> usize {
    num_safe(400000, input)
}
