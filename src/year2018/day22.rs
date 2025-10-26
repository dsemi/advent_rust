use crate::utils::parsers::*;
use crate::utils::*;

fn parse(i: &mut &str) -> Result<(u32, C<usize>)> {
    (preceded("depth: ", u32), preceded("\ntarget: ", c(usize))).parse_next(i)
}

fn erosion_levels(depth: u32, target: C<usize>, pad: C<usize>) -> Grid<u32> {
    let mut arr: Grid<u32> = Grid::new(target.0 + pad.0 + 1, target.1 + pad.1 + 1);
    for x in 0..arr.rows {
        for y in 0..arr.cols {
            let geologic_index = if C(x, y) == target {
                0
            } else if x == 0 {
                y * 48271
            } else if y == 0 {
                x * 16807
            } else {
                (arr[(x - 1, y)] * arr[(x, y - 1)]) as usize
            };
            arr[(x, y)] = (geologic_index as u32 + depth) % 20183;
        }
    }
    arr
}

pub fn part1(input: &str) -> u32 {
    let (depth, target) = parse.read(input);
    erosion_levels(depth, target, C(0, 0)).iter().map(|&v| v % 3).sum()
}

pub fn part2(input: &str) -> u32 {
    const N: usize = 8;
    const TORCH: usize = 1;
    let (depth, target) = parse.read(input);
    let mut cave = erosion_levels(depth, target, C(50, 10)).transform(|el| {
        let mut arr = [u32::MAX; 3];
        arr[el as usize % 3] = 0;
        arr
    });
    let mut q: [_; N] = std::array::from_fn(|_| vec![]);
    q[0].push((C(0, 0), TORCH));
    cave[C(0, 0)][TORCH] = 0;

    for i in 0.. {
        while let Some((pos, tool)) = q[i % N].pop() {
            let time = cave[pos][tool];
            if pos == target && tool == TORCH {
                return time;
            }

            for pos in [pos - C(1, 0), pos + C(1, 0), pos - C(0, 1), pos + C(0, 1)] {
                if cave.in_bounds(pos) && time + 1 < cave[pos][tool] {
                    cave[pos][tool] = time + 1;
                    let f_score = time as usize + 1 + pos.dist(&target);
                    q[f_score % N].push((pos, tool));
                }
            }
            for tool in 0..3 {
                if time + 7 < cave[pos][tool] {
                    cave[pos][tool] = time + 7;
                    let f_score = time as usize + 7 + pos.dist(&target);
                    q[f_score % N].push((pos, tool));
                }
            }
        }
    }
    unreachable!()
}
