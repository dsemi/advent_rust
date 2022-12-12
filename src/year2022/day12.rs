use crate::utils::*;

fn neighbors(grid: &Vec<Vec<u8>>, pos: &Coord<i32>) -> Vec<Coord<i32>> {
    let curr_h = grid[*pos];
    [
        Coord::new(-1, 0),
        Coord::new(1, 0),
        Coord::new(0, -1),
        Coord::new(0, 1),
    ]
    .into_iter()
    .filter_map(|c| {
        let pos2 = *pos + c;
        (pos2.x >= 0
            && pos2.x < grid.len() as i32
            && pos2.y >= 0
            && pos2.y < grid[0].len() as i32
            && grid[pos2] <= curr_h + 1)
            .then(|| pos2)
    })
    .collect()
}

fn solve(input: &str, p2: bool) -> usize {
    let mut starts = Vec::new();
    let mut done = Coord::new(0, 0);
    let mut grid = Vec::new();
    for (r, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for (c, v) in line.chars().enumerate() {
            match v {
                'S' => {
                    starts.push(Coord::new(r as i32, c as i32));
                    grid.last_mut().unwrap().push(0);
                }
                'E' => {
                    done = Coord::new(r as i32, c as i32);
                    grid.last_mut().unwrap().push(25);
                }
                _ => {
                    if p2 && v == 'a' {
                        starts.push(Coord::new(r as i32, c as i32));
                    }
                    grid.last_mut().unwrap().push(v as u8 - b'a');
                }
            }
        }
    }
    for (d, p) in bfs_m(starts, |p| neighbors(&grid, p)) {
        if p == done {
            return d;
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}
