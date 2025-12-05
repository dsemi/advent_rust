use crate::utils::*;

fn adj(grid: &Grid<bool>, c: C<usize>) -> impl Iterator<Item = C<usize>> {
    adjacents(c).filter(|&i| grid.in_bounds(i))
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<_> = grid_from_iter(input.bytes(), b'\n', |b| b == b'@');
    grid.idx_iter().filter(|&(i, &v)| v && adj(&grid, i).filter(|&j| grid[j]).count() < 4).count()
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<_> = grid_from_iter(input.bytes(), b'\n', |b| b == b'@');
    let mut rolls = Grid::<u8>::new(grid.rows + 2, grid.cols + 2);
    let mut q: Vec<_> = grid
        .idx_iter()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| (i + C(1, 1), adj(&grid, i).filter(|&j| grid[j]).count() as u8))
        .inspect(|&(i, n)| rolls[i] = n)
        .filter_map(|(i, n)| (n < 4).then_some(i))
        .collect();
    let mut total = 0;
    while let Some(i) = q.pop() {
        total += 1;
        for a in adjacents(i) {
            if rolls[a] == 4 {
                q.push(a);
            }
            rolls[a] -= 1;
        }
    }
    total
}
