fn make_sat(serial_num: i64) -> Vec<Vec<i64>> {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; 301]; 301];
    for (y, row) in grid.iter_mut().enumerate().skip(1) {
        for (x, e) in row.iter_mut().enumerate().skip(1) {
            let rack_id = x as i64 + 10;
            let mut power_level = rack_id * y as i64;
            power_level += serial_num;
            power_level *= rack_id;
            power_level = (power_level / 100).rem_euclid(10);
            power_level -= 5;
            *e = power_level;
        }
    }
    for y in (1..grid.len()).rev() {
        for x in (1..grid[y].len()).rev() {
            if y + 1 < grid.len() {
                grid[y][x] += grid[y + 1][x];
            }
            if x + 1 < grid[y].len() {
                grid[y][x] += grid[y][x + 1];
            }
            if y + 1 < grid.len() && x + 1 < grid[y].len() {
                grid[y][x] -= grid[y + 1][x + 1];
            }
        }
    }
    grid
}

fn max_cell(size: usize, sat: &[Vec<i64>]) -> (usize, usize, i64) {
    (1..sat.len() - size)
        .flat_map(|y| {
            (1..sat[y].len() - size).map(move |x| {
                (
                    x,
                    y,
                    sat[y][x] - sat[y + size][x] - sat[y][x + size] + sat[y + size][x + size],
                )
            })
        })
        .max_by_key(|x| x.2)
        .unwrap()
}

pub fn part1(input: i64) -> (usize, usize) {
    let sat = make_sat(input);
    let (x, y, _) = max_cell(3, &sat);
    (x, y)
}

pub fn part2(input: i64) -> (usize, usize, usize) {
    let sat = make_sat(input);
    (1..300)
        .map(|i| {
            let (x, y, c) = max_cell(i, &sat);
            ((x, y, i), c)
        })
        .max_by_key(|x| x.1)
        .unwrap()
        .0
}
