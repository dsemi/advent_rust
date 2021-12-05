use crate::utils::*;
use crate::year2018::day22::Tool::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::max;

#[derive(Clone, Copy, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd, ToPrimitive)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: Coord<i32>,
    tool: Tool,
}

fn next(t: &Tool) -> Tool {
    match t {
        Neither => Torch,
        Torch => ClimbingGear,
        ClimbingGear => Neither,
    }
}

fn parse(input: &str) -> (i32, Coord<i32>) {
    let lns = input.lines().collect::<Vec<_>>();
    let pts = lns[1]
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    (
        lns[0].split_once(' ').unwrap().1.parse().unwrap(),
        Coord::new(pts[0], pts[1]),
    )
}

fn erosion_levels(depth: i32, target: Coord<i32>) -> Vec<Vec<Tool>> {
    let mx = max(target.x, target.y) as usize + 3; // Arbitrary buffer size for search
    let mut arr = vec![vec![0; mx]; mx];
    for x in 0..mx {
        for y in 0..mx {
            let geologic_index = if Coord::new(x as i32, y as i32) == target {
                0
            } else if x == 0 {
                y * 48271
            } else if y == 0 {
                x * 16807
            } else {
                arr[x - 1][y] * arr[x][y - 1]
            };
            arr[x][y] = (geologic_index + depth as usize) % 20183;
        }
    }
    arr.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|v| FromPrimitive::from_usize(v % 3).unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let (depth, target) = parse(input);
    erosion_levels(depth, target)
        .into_iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.into_iter().enumerate().filter_map(move |(y, v)| {
                (x as i32 <= target.x && y as i32 <= target.y)
                    .then(|| ToPrimitive::to_u32(&v).unwrap())
            })
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (depth, target) = parse(input);
    let els = erosion_levels(depth, target);

    fn neighbors(els: &[Vec<Tool>], node: &Node) -> Vec<Node> {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |d| {
                let n_node = Node {
                    pos: node.pos + Coord::new(d.0, d.1),
                    tool: node.tool,
                };
                (n_node.pos.x >= 0
                    && n_node.pos.x < els.len() as i32
                    && n_node.pos.y >= 0
                    && n_node.pos.y < els.len() as i32
                    && n_node.tool != els[n_node.pos.x as usize][n_node.pos.y as usize])
                    .then(|| n_node)
            })
            .chain(
                vec![next(&node.tool), next(&next(&node.tool))]
                    .into_iter()
                    .filter_map(move |t| {
                        let n_node = Node {
                            pos: node.pos,
                            tool: t,
                        };
                        (n_node.tool != els[n_node.pos.x as usize][n_node.pos.y as usize])
                            .then(|| n_node)
                    }),
            )
            .collect()
    }

    fn heur(target: &Coord<i32>, node: &Node) -> usize {
        let dist = *target - node.pos;
        dist.x.abs() as usize + dist.y.abs() as usize
    }

    fn time(a: &Node, b: &Node) -> usize {
        if a.tool == b.tool {
            1
        } else {
            7
        }
    }

    let path = a_star(
        |n| neighbors(&els, n),
        time,
        |n| heur(&target, n),
        |n| {
            n == &Node {
                pos: target,
                tool: Torch,
            }
        },
        Node {
            pos: Coord::new(0, 0),
            tool: Torch,
        },
    )
    .unwrap();
    path.windows(2).map(|w| time(&w[0], &w[1])).sum()
}
