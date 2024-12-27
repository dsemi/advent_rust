use crate::utils::*;

type PosDir = (C<i32>, C<i32>);

fn idx(C(a, b): C<i32>) -> usize {
    (((a as u8 & 3).div_ceil(2) << 2) | (b as u8 & 3).div_ceil(2)).leading_zeros() as usize - 4
}

fn dijkstra(grid: &Grid<u8, i32>) -> (PosDir, Grid<[usize; 4], i32>) {
    let start = grid.position(|&v| v == b'S').unwrap();
    let end = grid.position(|&v| v == b'E').unwrap();
    let mut q: [_; 1002] = std::array::from_fn(|_| vec![]);
    let mut seen = grid.same_size_with([usize::MAX; 4]);
    q[0].push((0, start, C(0, 1)));
    seen[start][idx(C(0, 1))] = 0;
    for i in 0.. {
        while let Some((dist, pos, dir)) = q[i % q.len()].pop() {
            if pos == end {
                return ((pos, dir), seen);
            }
            for (len, dir) in [(1, dir), (1001, dir * C(0, 1)), (1001, dir * C(0, -1))] {
                let dist = dist + len;
                let pos = pos + dir;
                let idx = idx(dir);
                if grid[pos] != b'#' && dist < seen[pos][idx] {
                    q[dist % q.len()].push((dist, pos, dir));
                    seen[pos][idx] = dist;
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    let grid = input.bytes().collect();
    let ((pos, dir), scores) = dijkstra(&grid);
    scores[pos][idx(dir)]
}

pub fn part2(input: &str) -> usize {
    let grid = input.bytes().collect();
    let (k, scores) = dijkstra(&grid);
    let mut sqs = grid.same_size_with(false);
    let mut stack = vec![(k, scores[k.0][idx(k.1)])];
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
