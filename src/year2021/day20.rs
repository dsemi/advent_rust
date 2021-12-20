fn pad<T: Clone + Copy>(grid: &mut Vec<Vec<T>>, c: T) {
    grid.insert(0, vec![c; grid[0].len()]);
    grid.push(vec![c; grid[0].len()]);
    for row in grid.iter_mut() {
        row.insert(0, c);
        row.push(c);
    }
}

fn enhance(iea: &[u8], grid: &mut Vec<Vec<u8>>) {
    pad(grid, grid[0][0]);
    let mut grid2 = grid.clone();
    for r in 1..grid.len() - 1 {
        for c in 1..grid[r].len() - 1 {
            let idx = adj_w_diag!(grid, r, c)
                .into_iter()
                .fold(0, |a, b| a << 1 | (b == b'#') as usize);
            grid2[r][c] = iea[idx];
        }
    }
    std::mem::swap(grid, &mut grid2);
    let ch = iea[vec![grid[0][0]; 9]
        .into_iter()
        .fold(0, |a, b| a << 1 | (b == b'#') as usize)];
    let last = grid.len() - 1;
    for (i, row) in grid.iter_mut().enumerate() {
        row[0] = ch;
        *row.last_mut().unwrap() = ch;
        if i == 0 || i == last {
            row.iter_mut().for_each(|v| *v = ch);
        }
    }
}

fn run(input: &str, times: usize) -> usize {
    let (iea, img) = input.split_once("\n\n").unwrap();
    let mut im: Vec<Vec<u8>> = img.lines().map(|line| line.bytes().collect()).collect();
    pad(&mut im, b'.');
    (0..times).for_each(|_| enhance(iea.as_bytes(), &mut im));
    im.into_iter()
        .map(|row| row.into_iter().filter(|&v| v == b'#').count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    run(input, 2)
}

pub fn part2(input: &str) -> usize {
    run(input, 50)
}
