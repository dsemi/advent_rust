use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn conv(c: char) -> u32 {
    1 << (c as u32 - 'a' as u32)
}

#[derive(Clone, Copy)]
struct Edge {
    dest: usize,
    doors: u32,
    keys: u32,
    len: usize,
}

struct Maze {
    grid: Vec<char>,
    cols: usize,
    moves: HashMap<usize, Vec<Edge>>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap().len();
        Maze {
            grid: input.lines().flat_map(|line| line.chars()).collect(),
            cols: cols,
            moves: HashMap::new(),
        }
    }

    fn available_moves(&mut self, from: usize) -> &Vec<Edge> {
        if self.moves.get(&from).is_none() {
            let mut moves = Vec::new();
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(Edge {
                dest: from,
                doors: 0,
                keys: 0,
                len: 0,
            });

            while let Some(edge) = queue.pop_front() {
                if !visited.insert(edge.dest) {
                    continue;
                }
                for p in vec![
                    edge.dest - self.cols,
                    edge.dest - 1,
                    edge.dest + 1,
                    edge.dest + self.cols,
                ]
                .into_iter()
                .filter(|&p| p != from && self.grid[p] != '#')
                {
                    let mut edge = Edge {
                        dest: p,
                        doors: edge.doors,
                        keys: edge.keys,
                        len: edge.len + 1,
                    };
                    match self.grid[p] {
                        ch @ 'a'..='z' => {
                            edge.keys |= conv(ch);
                            moves.push(edge);
                        }
                        ch @ 'A'..='Z' => {
                            edge.doors |= conv(ch.to_ascii_lowercase());
                        }
                        _ => {}
                    }
                    queue.push_back(edge);
                }
            }
            self.moves.insert(from, moves);
        }
        &self.moves[&from]
    }
}

#[derive(Eq, PartialEq)]
struct State {
    poss: Vec<usize>,
    keys: u32,
    len: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(mut maze: Maze, start: char) -> Option<usize> {
    let start_poss = maze
        .grid
        .iter()
        .enumerate()
        .filter_map(|(i, v)| (v == &start).then(|| i))
        .collect();
    let ks = maze
        .grid
        .iter()
        .filter_map(|&v| v.is_ascii_lowercase().then(|| conv(v)))
        .fold(0, |a, b| a | b);
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut dists: HashMap<(Vec<usize>, u32), usize> = HashMap::new();
    queue.push(Reverse(State {
        poss: start_poss,
        keys: 0,
        len: 0,
    }));

    while let Some(Reverse(state)) = queue.pop() {
        if state.keys == ks {
            return Some(state.len);
        }
        let shortest = dists
            .entry((state.poss.clone(), state.keys))
            .or_insert(state.len);
        if state.len <= *shortest {
            *shortest = state.len;
            for (i, p) in state.poss.iter().enumerate() {
                for edge in maze.available_moves(*p).iter().filter(|edge| {
                    state.keys & edge.doors == edge.doors && state.keys & edge.keys != edge.keys
                }) {
                    let mut poss = state.poss.clone();
                    poss[i] = edge.dest;
                    let keys = state.keys | edge.keys;
                    let len = state.len + edge.len;
                    let shortest = dists.entry((poss.clone(), keys)).or_insert(len + 1);
                    if len < *shortest {
                        *shortest = len;
                        queue.push(Reverse(State {
                            poss: poss,
                            keys: keys,
                            len: len,
                        }));
                    }
                }
            }
        }
    }

    None
}

pub fn part1(input: &str) -> Option<usize> {
    search(Maze::new(input), '@')
}

pub fn part2(input: &str) -> Option<usize> {
    let mut maze = Maze::new(input);
    for (k, v) in (39..=41)
        .flat_map(|x| (39..=41).map(move |y| (x, y)))
        .zip("@#@###@#@".chars())
    {
        maze.grid[k.0 * maze.cols + k.1] = v;
    }
    search(maze, '@')
}
