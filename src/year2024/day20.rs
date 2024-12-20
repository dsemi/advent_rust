use crate::utils::*;

fn solve(input: &str, cheat_time: i32) -> usize {
    let grid: Grid<u8, i32> = input.bytes().collect();
    let start = grid.position(|&v| v == b'S').unwrap();
    let mut times = grid.same_size_with(0);
    std::iter::successors(Some((start, start, 1)), |&(prev, p, time)| {
        [C(p.0, p.1 - 1), C(p.0, p.1 + 1), C(p.0 + 1, p.1), C(p.0 - 1, p.1)]
            .into_iter()
            .find(|&pos| pos != prev && grid[pos] != b'#')
            .map(|pos| (p, pos, time + 1))
    })
    .for_each(|(_, p, time)| times[p] = time);
    let mut total = 0;
    for (p, &t) in times.idx_iter().filter(|&(_, &t)| t > 0) {
        let f = |d| (*times.get(p + d).unwrap_or(&0) >= t + C(0, 0).dist(&d) + 100) as usize;
        (2..=cheat_time).for_each(|c| total += f(C(0, c)) + f(C(0, -c)));
        for r in 1..=cheat_time {
            total += f(C(r, 0)) + f(C(-r, 0));
            for c in 1..=cheat_time - r {
                total += f(C(r, c)) + f(C(r, -c)) + f(C(-r, c)) + f(C(-r, -c));
            }
        }
    }
    total
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 20)
}
