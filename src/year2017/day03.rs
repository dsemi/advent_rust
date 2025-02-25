use crate::utils::*;
use hashbrown::HashMap;
use itertools::Itertools;

fn mid_pt(x: i64, y: i64) -> i64 {
    (x + y) / 2
}

fn corners() -> impl Iterator<Item = i64> {
    (1..)
        .flat_map(|i| vec![i; 2])
        .scan(1, |a, b| Some(std::mem::replace(a, *a + b)))
}

pub fn part1(n: i64) -> i64 {
    let ns: Vec<i64> = corners().take_while_inclusive(|&c| c < n).collect();
    let a = ns[ns.len() - 1];
    let b = ns[ns.len() - 2];
    let c = ns[ns.len() - 3];
    b - mid_pt(b, c) + (n - mid_pt(a, b)).abs()
}

fn spiral_path() -> impl Iterator<Item = i64> {
    let mut tbl = HashMap::new();
    tbl.insert(C(0, 0), 1);
    (1..)
        .flat_map(|i| vec![i; 2])
        .zip(
            vec![C(1, 0), C(0, 1), C(-1, 0), C(0, -1)]
                .into_iter()
                .cycle(),
        )
        .flat_map(|(n, d)| vec![d; n])
        .scan((tbl, C(0, 0)), |(m, pos), dir| {
            *pos += dir;
            let val = adjacents(*pos).map(|c| *m.get(&c).unwrap_or(&0)).sum();
            m.insert(*pos, val);
            Some(val)
        })
}

pub fn part2(n: i64) -> Option<i64> {
    spiral_path().find(|&x| x > n)
}
