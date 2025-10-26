use crate::utils::*;

fn neighbors(n: i32, pos: C<i32>) -> Vec<C<i32>> {
    vec![pos + C(1, 0), pos + C(-1, 0), pos + C(0, 1), pos + C(0, -1)]
        .into_iter()
        .filter(|&C(x, y)| {
            x >= 0
                && y >= 0
                && (x * x + 3 * x + 2 * x * y + y + y * y + n).count_ones().is_multiple_of(2)
        })
        .collect()
}

pub fn part1(n: i32) -> Option<usize> {
    let target = C(31, 39);
    bfs(C(1, 1), move |p| neighbors(n, *p)).filter(|x| x.1 == target).map(|x| x.0).next()
}

pub fn part2(n: i32) -> usize {
    bfs(C(1, 1), |p| neighbors(n, *p)).take_while(|x| x.0 <= 50).count()
}
