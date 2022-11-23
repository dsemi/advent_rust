fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

fn dijkstra(grid: Vec<Vec<i8>>) -> usize {
    let dim = grid.len() + 2;
    let mut lookup = vec![0; dim * dim];
    for (r, row) in grid.into_iter().enumerate() {
        for (c, v) in row.into_iter().enumerate() {
            lookup[dim * (r + 1) + c + 1] = v;
        }
    }
    let goal = dim * dim - dim - 2;
    let mut q = vec![vec![]; 16];
    let mut tmp = vec![];
    q[0].push(dim + 1);
    for qi in 0.. {
        tmp.clear();
        std::mem::swap(&mut q[qi % 16], &mut tmp);
        for p in tmp.iter().copied() {
            if lookup[p] < 1 {
                continue;
            }
            if p == goal {
                return qi;
            }
            lookup[p] *= -1;
            for n in [p - 1, p + 1, p - dim, p + dim] {
                if lookup[n] >= 1 {
                    q[(qi + lookup[n] as usize) % 16].push(n);
                }
            }
        }
    }
    unreachable!();
}

pub fn part1(input: &str) -> usize {
    dijkstra(parse(input))
}

pub fn part2(input: &str) -> usize {
    let small_grid = parse(input);
    let mut grid = vec![vec![0; 5 * small_grid[0].len()]; 5 * small_grid.len()];
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            let (rd, rm) = (r / small_grid.len(), r % small_grid.len());
            let (cd, cm) = (c / small_grid[0].len(), c % small_grid[0].len());
            *v = ((small_grid[rm][cm] as usize - 1 + rd + cd) % 9 + 1) as i8;
        }
    }
    dijkstra(grid)
}
