use std::cmp::max;

fn flash(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) -> usize {
    grid[r][c] = -1;
    let mut flashes = 1;
    for r in [r - 1, r, r + 1] {
        for c in [c - 1, c, c + 1] {
            if grid.get(r).and_then(|row| row.get(c)).unwrap_or(&-1) != &-1 {
                grid[r][c] += 1;
                if grid[r][c] > 9 {
                    flashes += flash(grid, r, c);
                }
            }
        }
    }
    flashes
}

fn run(input: &str) -> impl Iterator<Item = usize> {
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.bytes().map(|c| (c - b'0') as i32).collect())
        .collect();
    std::iter::repeat_with(move || {
        grid.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v = max(0, *v) + 1));
        (0..grid.len())
            .map(|r| {
                (0..grid[r].len())
                    .filter_map(|c| (grid[r][c] > 9).then(|| flash(&mut grid, r, c)))
                    .sum::<usize>()
            })
            .sum()
    })
}

pub fn part1(input: &str) -> usize {
    run(input).take(100).sum()
}

pub fn part2(input: &str) -> Option<usize> {
    run(input).position(|x| x == 100).map(|x| x + 1)
}
