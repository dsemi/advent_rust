fn count_trees(right: usize, down: usize, s: &str) -> i64 {
    let grid: Vec<Vec<char>> = s.lines().map(|x| x.chars().collect()).collect();
    grid.iter()
        .step_by(down)
        .enumerate()
        .map(|(y, row)| (row[y * right % row.len()] == '#') as i64)
        .sum()
}

pub fn part1(input: &str) -> i64 {
    count_trees(3, 1, input)
}

pub fn part2(input: &str) -> i64 {
    let steps: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    steps.into_iter()
        .map(|(right, down)| count_trees(right, down, input))
        .product()
}
