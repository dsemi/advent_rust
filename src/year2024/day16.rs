use crate::utils::*;

type PosDir = (C<i32>, C<i32>);

fn neighbors(grid: &Grid<u8, i32>, (pos, dir): PosDir) -> Vec<(usize, PosDir)> {
    [(1, dir), (1001, dir * C(0, 1)), (1001, dir * C(0, -1))]
        .into_iter()
        .map(|(dist, dir)| (dist, (pos + dir, dir)))
        .filter(|(_, (pos, _))| grid[*pos] != b'#')
        .collect()
}

pub fn part1(input: &str) -> Option<usize> {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let start = (grid.position(|&v| v == b'S').unwrap(), C(0, 1));
    dijkstra(start, |&posdir| neighbors(&grid, posdir))
        .find_map(|(score, (pos, _))| (grid[pos] == b'E').then_some(score))
}

fn idx(C(a, b): C<i32>) -> usize {
    (((a as u8 & 3).div_ceil(2) << 2) | (b as u8 & 3).div_ceil(2)).leading_zeros() as usize - 4
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let start = (grid.position(|&v| v == b'S').unwrap(), C(0, 1));
    let mut scores = grid.same_size_with([usize::MAX; 4]);
    scores[start.0][idx(start.1)] = 0;
    let (score, k) = dijkstra(start, |&posdir| neighbors(&grid, posdir))
        .find(|&(score, (pos, dir))| {
            scores[pos][idx(dir)] = scores[pos][idx(dir)].min(score);
            grid[pos] == b'E'
        })
        .unwrap();
    let mut sqs = grid.same_size_with(false);
    let mut stack = vec![(k, score)];
    while let Some(((pos, dir), score)) = stack.pop() {
        sqs[pos] = true;
        [
            ((pos - dir, dir), score - 1),
            ((pos - dir, dir * C(0, 1)), score - 1001),
            ((pos - dir, dir * C(0, -1)), score - 1001),
        ]
        .into_iter()
        .filter(|((pos, _), _)| grid[*pos] != b'#')
        .filter(|&((pos, dir), s)| scores[pos][idx(dir)] == s)
        .for_each(|e| stack.push(e));
    }
    sqs.into_iter().filter(|&b| b).count()
}
