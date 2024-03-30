fn count_trees(right: usize, down: usize, s: &str) -> usize {
    s.lines()
        .map(|x| x.as_bytes())
        .step_by(down)
        .enumerate()
        .map(|(y, row)| (row[y * right % row.len()] == b'#') as usize)
        .sum()
}

pub fn part1(input: &str) -> usize {
    count_trees(3, 1, input)
}

pub fn part2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| count_trees(right, down, input))
        .product()
}
