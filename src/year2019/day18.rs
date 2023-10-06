use crate::utils::*;
use ahash::AHashMap;

fn conv(c: char) -> u32 {
    1 << (c as u32 - 'a' as u32)
}

#[derive(Clone, Copy)]
struct Edge {
    dest: usize,
    doors: u32,
    keys: u32,
}

#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Start,
    Key(u32),
    Door(u32),
}

fn tile(c: char) -> Tile {
    match c {
        'a'..='z' => Tile::Key(conv(c)),
        'A'..='Z' => Tile::Door(conv(c.to_ascii_lowercase())),
        '@' => Tile::Start,
        '#' => Tile::Wall,
        _ => Tile::Floor,
    }
}

struct Maze {
    grid: Vec<Tile>,
    cols: usize,
    moves: AHashMap<usize, Vec<(usize, Edge)>>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Node {
    poss: Vec<usize>,
    keys: u32,
}

impl Maze {
    fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().len();
        Maze {
            grid: input
                .lines()
                .flat_map(|line| line.chars().map(tile))
                .collect(),
            cols,
            moves: AHashMap::new(),
        }
    }

    fn available_moves(&mut self, node: &Node) -> Vec<(usize, Node)> {
        node.poss
            .iter()
            .enumerate()
            .flat_map(|(i, from)| {
                if !self.moves.contains_key(from) {
                    let moves = bfs_on(
                        |e| e.dest,
                        [Edge {
                            dest: *from,
                            doors: 0,
                            keys: 0,
                        }],
                        |edge| {
                            vec![
                                edge.dest - self.cols,
                                edge.dest - 1,
                                edge.dest + 1,
                                edge.dest + self.cols,
                            ]
                            .into_iter()
                            .filter(|&p| p != *from && self.grid[p] != Tile::Wall)
                            .map(|p| {
                                let mut edge = Edge {
                                    dest: p,
                                    doors: edge.doors,
                                    keys: edge.keys,
                                };
                                match self.grid[p] {
                                    Tile::Key(k) => {
                                        edge.keys |= k;
                                    }
                                    Tile::Door(k) => {
                                        edge.doors |= k;
                                    }
                                    _ => {}
                                }
                                edge
                            })
                            .collect::<Vec<_>>()
                        },
                    )
                    .filter(|(_, edge)| matches!(self.grid[edge.dest], Tile::Key(_)))
                    .collect();
                    self.moves.insert(*from, moves);
                }
                self.moves[from]
                    .iter()
                    .filter(|&(_, edge)| {
                        node.keys & edge.doors == edge.doors && node.keys & edge.keys != edge.keys
                    })
                    .map(move |(len, edge)| {
                        let mut poss = node.poss.clone();
                        poss[i] = edge.dest;
                        (
                            *len,
                            Node {
                                poss,
                                keys: node.keys | edge.keys,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn search(mut maze: Maze) -> Option<usize> {
    let start_poss = maze
        .grid
        .iter()
        .enumerate()
        .filter(|&(_, v)| v == &Tile::Start)
        .map(|(i, _)| i)
        .collect();
    let ks = maze
        .grid
        .iter()
        .filter_map(|v| match v {
            Tile::Key(k) => Some(k),
            _ => None,
        })
        .fold(0, |a, b| a | b);
    dijkstra(
        Node {
            poss: start_poss,
            keys: 0,
        },
        |n| maze.available_moves(n),
    )
    .find_map(|(d, n)| (n.keys == ks).then(|| d))
}

pub fn part1(input: &str) -> Option<usize> {
    search(Maze::new(input))
}

pub fn part2(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    for (k, v) in (39..=41)
        .flat_map(|x| (39..=41).map(move |y| (x, y)))
        .zip("@#@###@#@".chars().map(tile))
    {
        maze.grid[k.0 * maze.cols + k.1] = v;
    }
    search(maze)
}
