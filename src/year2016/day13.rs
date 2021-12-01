use crate::utils::*;

fn neighbors(n: i32, pos: Coord<i32>) -> Vec<Coord<i32>> {
    vec![
        pos + Coord::new(1, 0),
        pos + Coord::new(-1, 0),
        pos + Coord::new(0, 1),
        pos + Coord::new(0, -1),
    ]
    .into_iter()
    .filter_map(|p| {
        let x = p.x;
        let y = p.y;
        (x >= 0 && y >= 0 && (x * x + 3 * x + 2 * x * y + y + y * y + n).count_ones() % 2 == 0).then(|| p)
    })
    .collect()
}

pub fn part1(n: i32) -> Option<usize> {
    let target = Coord::new(31, 39);
    bfs(Coord::new(1, 1), move |p| neighbors(n, *p))
        .filter(|x| x.1 == target)
        .map(|x| x.0)
        .next()
}

pub fn part2(n: i32) -> usize {
    bfs(Coord::new(1, 1), |p| neighbors(n, *p))
        .take_while(|x| x.0 <= 50)
        .count()
}
