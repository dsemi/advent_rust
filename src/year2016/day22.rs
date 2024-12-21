use crate::utils::parsers::*;
use crate::utils::*;

#[derive(Clone, Default)]
struct Node {
    used: i64,
    avail: i64,
}

fn node(i: &mut &str) -> PResult<(C<i32>, Node)> {
    "/dev/grid/node-x".parse_next(i)?;
    let coord = sep2(i32, "-y").output_into().parse_next(i)?;
    delimited(space1, i64, 'T').parse_next(i)?;
    let used = delimited(space1, i64, 'T').parse_next(i)?;
    let avail = delimited(space1, i64, 'T').parse_next(i)?;
    delimited(space1, i64, '%').parse_next(i)?;
    Ok((coord, Node { used, avail }))
}

pub fn part1(input: &str) -> usize {
    let nodes: Vec<Node> = input.lines().skip(2).map(|line| node.read(line).1).collect();
    let mut total = 0;
    for (i, a) in nodes.iter().enumerate() {
        for b in nodes.iter().skip(i + 1) {
            total += ((1..b.avail).contains(&a.used) || (1..a.avail).contains(&b.used)) as usize;
        }
    }
    total
}

fn neighbors(grid: &Grid<i64, i32>, &(opn, goal): &(C<i32>, C<i32>)) -> Vec<(C<i32>, C<i32>)> {
    vec![opn + C(0, 1), opn + C(0, -1), opn + C(1, 0), opn + C(-1, 0)]
        .into_iter()
        .filter(|&o2| *grid.get(o2).unwrap_or(&i64::MAX) <= 100)
        .map(|o2| (o2, if o2 == goal { opn } else { goal }))
        .collect()
}

pub fn part2(input: &str) -> Option<usize> {
    let max_coord = node.read(input.lines().last().unwrap()).0;
    let mut grid = Grid::new_with(max_coord.0 + 1, max_coord.1 + 1, 0);
    input.lines().skip(2).map(|line| node.read(line)).for_each(|(i, node)| grid[i] = node.used);
    let opn = grid.position(|&used| used == 0).unwrap();
    let mx = C(grid.rows - 1, 0);
    bfs2((opn, mx), move |st| neighbors(&grid, st)).find_map(|(d, v)| (v.1 == C(0, 0)).then_some(d))
}
