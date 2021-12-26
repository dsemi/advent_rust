pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut changed = true;
    let mut c = 0;
    while std::mem::replace(&mut changed, false) {
        let grid2 = grid.clone();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid2[r][c] == '>' {
                    let next = (c + 1) % grid[r].len();
                    if grid2[r][next] == '.' {
                        changed = true;
                        grid[r][c] = '.';
                        grid[r][next] = '>';
                    }
                }
            }
        }
        let grid2 = grid.clone();
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid2[r][c] == 'v' {
                    let next = (r + 1) % grid.len();
                    if grid2[next][c] == '.' {
                        changed = true;
                        grid[r][c] = '.';
                        grid[next][c] = 'v';
                    }
                }
            }
        }
        c += 1;
    }
    c
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}
