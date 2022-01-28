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

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> Option<usize> {
    let grid = parse(input);
    dijkstra(Coord::new(0, 0), |p| neighbs(&grid, p))
        .find(|x| x.1 == Coord::new(grid.len() - 1, grid.len() - 1))
        .map(|x| x.0)
}

pub fn part2(input: &str) -> Option<usize> {
    let small_grid = parse(input);
    let mut grid = vec![vec![0; 5 * small_grid[0].len()]; 5 * small_grid.len()];
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            let (rd, rm) = (r / small_grid.len(), r % small_grid.len());
            let (cd, cm) = (c / small_grid[0].len(), c % small_grid[0].len());
            *v = (small_grid[rm][cm] - 1 + rd + cd) % 9 + 1;
        }
    }
    dijkstra(Coord::new(0, 0), |p| neighbs(&grid, p))
        .find(|x| x.1 == Coord::new(grid.len() - 1, grid.len() - 1))
        .map(|x| x.0)
}
