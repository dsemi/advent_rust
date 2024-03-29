use crate::utils::parsers::*;
use crate::utils::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::max;
use Tool::*;

#[derive(Clone, Copy, Eq, FromPrimitive, Hash, PartialEq, ToPrimitive)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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

fn parse(i: &mut &str) -> PResult<(i32, C<i32>)> {
    (preceded("depth: ", i32), preceded("\ntarget: ", c(i32))).parse_next(i)
}

fn erosion_levels(depth: i32, target: C<i32>) -> Grid<Tool, i32> {
    let mx = max(target.0, target.1) + 3; // Arbitrary buffer size for search
    let mut arr: Grid<usize, i32> = Grid::new(mx, mx);
    for x in 0..mx {
        for y in 0..mx {
            let geologic_index = if C(x, y) == target {
                0
            } else if x == 0 {
                y as usize * 48271
            } else if y == 0 {
                x as usize * 16807
            } else {
                arr[(x - 1, y)] * arr[(x, y - 1)]
            };
            arr[(x, y)] = (geologic_index + depth as usize) % 20183;
        }
    }
    arr.transform(|v| FromPrimitive::from_usize(v % 3).unwrap())
}

pub fn part1(input: &str) -> u32 {
    let (depth, target) = parse.read(input);
    erosion_levels(depth, target)
        .into_idx_iter()
        .filter(|&(C(x, y), _)| x <= target.0 && y <= target.1)
        .map(|(_, v)| ToPrimitive::to_u32(&v).unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (depth, target) = parse.read(input);
    let els = erosion_levels(depth, target);

    fn neighbors(els: &Grid<Tool, i32>, node: &Node) -> Vec<(usize, Node)> {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |d| {
                let n_node = Node {
                    pos: node.pos + C(d.0, d.1),
                    tool: node.tool,
                };
                els.get(n_node.pos)
                    .filter(|&tool| *tool != n_node.tool)
                    .map(|_| (1, n_node))
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
