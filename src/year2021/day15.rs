use crate::utils::*;

fn neighbs<'a>(
    grid: &'a [Vec<usize>],
    pos: &Coord<usize>,
) -> impl Iterator<Item = (usize, Coord<usize>)> + 'a {
    [
        *pos + Coord::new(1, 0),
        *pos + Coord::new(0, 1),
        *pos - Coord::new(1, 0),
        *pos - Coord::new(0, 1),
    ]
    .into_iter()
    .filter_map(|p| (p.x < grid.len() && p.y < grid[p.x].len()).then(|| (grid[p.x][p.y], p)))
}

pub fn part1(input: &str) -> Option<usize> {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    dijkstra(Coord::new(0, 0), |p| neighbs(&grid, p))
        .find(|x| x.1 == Coord::new(grid.len() - 1, grid.len() - 1))
        .map(|x| x.0)
}

pub fn part2(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    for row in grid.iter_mut() {
        let row_init = row.clone();
        (1..5).for_each(|i| row.extend(row_init.iter().map(|x| (x + i - 1) % 9 + 1)));
    }
    let grid_init = grid.clone();
    for i in 1..5 {
        for row in &grid_init {
            grid.push(row.iter().map(|x| (x + i - 1) % 9 + 1).collect());
        }
    }
    dijkstra(Coord::new(0, 0), |p| neighbs(&grid, p))
        .find(|x| x.1 == Coord::new(grid.len() - 1, grid.len() - 1))
        .map(|x| x.0)
}
