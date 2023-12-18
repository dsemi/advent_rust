use crate::utils::*;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.bytes().map(|x| (x - b'0') as usize).collect())
        .collect()
}

type St = (C<i32>, C<i32>, usize);

fn neighbors<const LO: usize, const HI: usize>(
    grid: &[Vec<usize>],
    (pos, dir, consec): St,
) -> impl Iterator<Item = (usize, St)> + '_ {
    let mut res = vec![(pos + dir, dir, consec + 1)];
    // == 0 for starting pos
    if consec == 0 || consec >= LO {
        res.push((pos + dir * C(0, 1), dir * C(0, 1), 1));
        res.push((pos + dir * C(0, -1), dir * C(0, -1), 1));
    }
    res.into_iter()
        .filter(|(_, _, c)| *c <= HI)
        .filter_map(|st| grid.get_cell(st.0).map(|d| (*d, st)))
}

pub fn part1(input: &str) -> Option<usize> {
    let grid = parse(input);
    let end = C(grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    dijkstra((C(0, 0), C(0, 1), 0), |st| neighbors::<1, 3>(&grid, *st))
        .find_map(|(d, (pos, _, _))| (pos == end).then_some(d))
}

pub fn part2(input: &str) -> Option<usize> {
    let grid = parse(input);
    let end = C(grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    dijkstra((C(0, 0), C(0, 1), 0), |st| neighbors::<4, 10>(&grid, *st))
        .find_map(|(d, (pos, _, c))| (pos == end && c >= 4).then_some(d))
}
