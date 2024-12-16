use crate::utils::*;
use hashbrown::{HashMap, HashSet};

type PosDir = (C<i32>, C<i32>);

fn neighbors(
    parents: &mut HashMap<PosDir, Vec<PosDir>>,
    grid: &Grid<u8, i32>,
    (pos, dir): PosDir,
) -> Vec<(usize, PosDir)> {
    [(1, dir), (1001, dir * C(0, 1)), (1001, dir * C(0, -1))]
        .into_iter()
        .map(|(dist, dir)| (dist, (pos + dir, dir)))
        .filter(|(_, (pos, _))| grid[*pos] != b'#')
        .inspect(|&(_, n)| parents.entry(n).or_insert_with(Vec::new).push((pos, dir)))
        .collect()
}

pub fn part1(input: &str) -> Option<usize> {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let start = (grid.position(|&v| v == b'S').unwrap(), C(0, 1));
    let mut parents = HashMap::new();
    dijkstra(start, |&posdir| neighbors(&mut parents, &grid, posdir))
        .find_map(|(score, (pos, _))| (grid[pos] == b'E').then_some(score))
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let start = (grid.position(|&v| v == b'S').unwrap(), C(0, 1));
    let mut parents = HashMap::new();
    let mut scores = HashMap::new();
    scores.insert(start, 0);
    let (score, k) = dijkstra(start, |&posdir| neighbors(&mut parents, &grid, posdir))
        .find(|&(score, k)| {
            scores.entry(k).or_insert(score);
            grid[k.0] == b'E'
        })
        .unwrap();
    let mut sqs = HashSet::new();
    let mut stack = vec![(k, score)];
    while let Some((k, score)) = stack.pop() {
        sqs.insert(k.0);
        parents
            .get(&k)
            .unwrap_or(&vec![])
            .iter()
            .filter(|&p| scores[p] == score - 1 || scores[p] == score - 1001)
            .for_each(|p| stack.push((*p, scores[p])));
    }
    sqs.len()
}
