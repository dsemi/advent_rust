fn make_grid(input: &str) -> [i32; 10000] {
    let mut grid = [0; 10000];
    for (i, row) in input.lines().enumerate() {
        for (j, v) in row.chars().enumerate() {
            grid[100 * i + j] = if v == '#' { 1 } else { 0 };
        }
    }
    grid
}

fn next_grid(grid: [i32; 10000]) -> [i32; 10000] {
    let mut grid2 = grid.clone();
    for i in 0..100 {
        for j in 0..100 {
            let adj_lights = vec![
                (i - 1, j - 1),
                (i, j - 1),
                (i + 1, j - 1),
                (i - 1, j),
                (i + 1, j),
                (i - 1, j + 1),
                (i, j + 1),
                (i + 1, j + 1),
            ]
            .into_iter()
            .filter(|&(x, y)| x < 100 && y < 100 && grid[100 * x + y] > 0)
            .count();
            if grid[100 * i + j] > 0 && adj_lights != 2 && adj_lights != 3 {
                grid2[100 * i + j] = 0;
            } else if grid[100 * i + j] == 0 && adj_lights == 3 {
                grid2[100 * i + j] = 1;
            }
        }
    }
    grid2
}

pub fn part1(input: &str) -> i32 {
    let mut grid = make_grid(input);
    for _ in 0..100 {
        grid = next_grid(grid);
    }
    grid.iter().sum()
}

pub fn part2(input: &str) -> i32 {
    let mut grid = make_grid(input);
    grid[0] = 1;
    grid[99] = 1;
    grid[9900] = 1;
    grid[9999] = 1;
    for _ in 0..100 {
        grid = next_grid(grid);
        grid[0] = 1;
        grid[99] = 1;
        grid[9900] = 1;
        grid[9999] = 1;
    }
    grid.iter().sum()
}
