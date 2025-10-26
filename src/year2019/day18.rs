use crate::utils::*;
use Tile::*;
use hashbrown::HashMap;

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
        'a'..='z' => Key(conv(c)),
        'A'..='Z' => Door(conv(c.to_ascii_lowercase())),
        '@' => Start,
        '#' => Wall,
        _ => Floor,
    }
}

struct Maze {
    grid: Vec<Tile>,
    cols: usize,
    moves: HashMap<usize, Vec<(usize, Edge)>>,
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
            grid: input.lines().flat_map(|line| line.chars().map(tile)).collect(),
            cols,
            moves: HashMap::new(),
        }
    }

    fn available_moves(&mut self, node: &Node) -> Vec<(usize, Node)> {
        node.poss
            .iter()
            .enumerate()
            .flat_map(|(i, &from)| {
                self.moves
                    .entry(from)
                    .or_insert_with(|| {
                        bfs_on(
                            |e| e.dest,
                            [Edge { dest: from, doors: 0, keys: 0 }],
                            |edge| {
                                vec![
                                    edge.dest - self.cols,
                                    edge.dest - 1,
                                    edge.dest + 1,
                                    edge.dest + self.cols,
                                ]
                                .into_iter()
                                .filter(|&p| p != from && self.grid[p] != Wall)
                                .map(|p| Edge {
                                    dest: p,
                                    doors: edge.doors
                                        | if let Door(k) = self.grid[p] { k } else { 0 },
                                    keys: edge.keys | if let Key(k) = self.grid[p] { k } else { 0 },
                                })
                                .collect::<Vec<_>>()
                            },
                        )
                        .filter(|(_, edge)| matches!(self.grid[edge.dest], Key(_)))
                        .collect()
                    })
                    .iter()
                    .filter(|(_, e)| node.keys & e.doors == e.doors && node.keys & e.keys != e.keys)
                    .map(|(len, edge)| {
                        let mut poss = node.poss.clone();
                        poss[i] = edge.dest;
                        (*len, Node { poss, keys: node.keys | edge.keys })
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn search(mut maze: Maze) -> Option<usize> {
    let start_poss =
        maze.grid.iter().enumerate().filter_map(|(i, v)| (v == &Start).then_some(i)).collect();
    let ks = maze
        .grid
        .iter()
        .filter_map(|v| match v {
            Key(k) => Some(k),
            _ => None,
        })
        .fold(0, |a, b| a | b);
    dijkstra(Node { poss: start_poss, keys: 0 }, |n| maze.available_moves(n))
        .find_map(|(d, n)| (n.keys == ks).then_some(d))
}

pub fn part1(input: &str) -> Option<usize> {
    search(Maze::new(input))
}

pub fn part2(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    for (k, v) in
        (39..=41).flat_map(|x| (39..=41).map(move |y| (x, y))).zip("@#@###@#@".chars().map(tile))
    {
        maze.grid[k.0 * maze.cols + k.1] = v;
    }
    search(maze)
}
