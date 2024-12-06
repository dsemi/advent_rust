use crate::utils::*;

fn to_bit(dir: C<i32>) -> u8 {
    match dir {
        C(1, 0) => 1,
        C(0, 1) => 2,
        C(-1, 0) => 4,
        C(0, -1) => 8,
        _ => unreachable!(),
    }
}

fn walk(
    grid: &Grid<char, i32>,
    mut pos: C<i32>,
    mut dir: C<i32>,
) -> Option<impl Iterator<Item = C<i32>>> {
    let mut visited = grid.same_size_with(0_u8);
    while grid.in_bounds(pos) {
        let b = to_bit(dir);
        if visited[pos] & b != 0 {
            return None;
        }
        visited[pos] |= b;
        if matches!(grid.get(pos + dir), Some('#')) {
            dir *= C(0, -1);
        } else {
            pos += dir;
        }
    }
    Some(visited.into_idx_iter().filter_map(|(i, v)| (v != 0).then_some(i)))
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<char, i32> = input.chars().collect();
    let pos = grid.idx_iter().find_map(|(i, &v)| (v == '^').then_some(i)).unwrap();
    walk(&grid, pos, C(-1, 0)).unwrap().count()
}

pub fn part2(input: &str) -> usize {
    let mut grid: Grid<char, i32> = input.chars().collect();
    let pos = grid.idx_iter().find_map(|(i, &v)| (v == '^').then_some(i)).unwrap();
    walk(&grid, pos, C(-1, 0))
        .unwrap()
        .filter(|&idx| {
            grid[idx] == '.' && {
                grid[idx] = '#';
                let loops = walk(&grid, pos, C(-1, 0)).is_none();
                grid[idx] = '.';
                loops
            }
        })
        .count()
}
