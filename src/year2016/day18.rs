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
    (0..n)
        .scan(input.chars().collect::<Vec<_>>(), |state, _| {
            let res = state.iter().filter(|&x| *x == '.').count();
            *state = (0..state.len())
                .map(|i| {
                    safe_or_trap(
                        *state.get(i - 1).unwrap_or(&'.'),
                        state[i],
                        *state.get(i + 1).unwrap_or(&'.'),
                    )
                })
                .collect();
            Some(res)
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    num_safe(40, input)
}

pub fn part2(input: &str) -> usize {
    num_safe(400000, input)
}
