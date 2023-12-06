use crate::utils::parsers::*;
use crate::utils::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::max;
use Tool::*;

#[derive(Clone, Copy, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd, ToPrimitive)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: C<i32>,
    tool: Tool,
}

fn next(t: &Tool) -> Tool {
    match t {
        Neither => Torch,
        Torch => ClimbingGear,
        ClimbingGear => Neither,
    }
}

fn parse(input: &str) -> (i32, C<i32>) {
    let lns = input.lines().collect::<Vec<_>>();
    let pts: Vec<_> = lns[1]
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(int)
        .collect();
    (lns[0].split_once(' ').unwrap().1.int(), C(pts[0], pts[1]))
}

fn erosion_levels(depth: i32, target: C<i32>) -> Vec<Vec<Tool>> {
    let mx = max(target.0, target.1) as usize + 3; // Arbitrary buffer size for search
    let mut arr = vec![vec![0; mx]; mx];
    for x in 0..mx {
        for y in 0..mx {
            let geologic_index = if C(x as i32, y as i32) == target {
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
            row.into_iter()
                .enumerate()
                .filter(move |(y, _)| x as i32 <= target.0 && *y as i32 <= target.1)
                .map(|(_, v)| ToPrimitive::to_u32(&v).unwrap())
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (depth, target) = parse(input);
    let els = erosion_levels(depth, target);

    fn neighbors(els: &[Vec<Tool>], node: &Node) -> Vec<(usize, Node)> {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |d| {
                let n_node = Node {
                    pos: node.pos + C(d.0, d.1),
                    tool: node.tool,
                };
                (n_node.pos.0 >= 0
                    && n_node.pos.0 < els.len() as i32
                    && n_node.pos.1 >= 0
                    && n_node.pos.1 < els.len() as i32
                    && n_node.tool != els[n_node.pos])
                    .then_some((1, n_node))
            })
            .chain(
                vec![next(&node.tool), next(&next(&node.tool))]
                    .into_iter()
                    .filter_map(move |t| {
                        let n_node = Node {
                            pos: node.pos,
                            tool: t,
                        };
                        (n_node.tool != els[n_node.pos]).then_some((7, n_node))
                    }),
            )
            .collect()
    }

    fn heur(target: &C<i32>, node: &Node) -> usize {
        target.dist(&node.pos) as usize
    }

    a_star(
        |n| neighbors(&els, n),
        |n| heur(&target, n),
        |n| {
            n == &Node {
                pos: target,
                tool: Torch,
            }
        },
        Node {
            pos: C(0, 0),
            tool: Torch,
        },
    )
    .unwrap()
    .last()
    .unwrap()
    .0
}
