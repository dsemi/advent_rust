use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn solve(s: &str, dim: usize) -> usize {
    let mut on_cubes: HashSet<Vec<i64>> = s
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter(|(_, v)| v == &'#').map(move |(x, _)| {
                let mut v = vec![x as i64, y as i64];
                v.extend(vec![0; dim - 2]);
                v
            })
        })
        .collect();
    let pts = vec![vec![-1, 0, 1]; dim];
    for _ in 0..6 {
        let mut m = HashMap::new();
        for pos in &on_cubes {
            for neighb in pts.iter().multi_cartesian_product() {
                let pos2: Vec<i64> = pos.iter().zip(neighb).map(|(a, b)| a + b).collect();
                if pos != &pos2 {
                    let e = m.entry(pos2).or_insert(0);
                    *e += 1;
                }
            }
        }
        let p1: HashSet<Vec<i64>> = on_cubes
            .iter()
            .filter(|&pos| (2..=3).contains(m.get(pos).unwrap_or(&0)))
            .cloned()
            .collect();
        let p2 = m
            .iter()
            .filter(|&(pos, v)| !on_cubes.contains(pos) && v == &3)
            .map(|(pos, _)| pos.clone())
            .collect();
        on_cubes = p1.union(&p2).cloned().collect();
    }
    on_cubes.len()
}

pub fn part1(input: &str) -> usize {
    solve(input, 3)
}

pub fn part2(input: &str) -> usize {
    solve(input, 4)
}
