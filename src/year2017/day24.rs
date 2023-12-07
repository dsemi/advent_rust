use crate::utils::parsers::*;
use ahash::AHashSet;
use std::cmp::max_by_key;

#[derive(Clone, Copy, Debug)]
struct Pipe {
    id: u64,
    a: u32,
    b: u32,
}

fn parse_pipes(input: &str) -> Vec<Pipe> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (a, b) = sep_tuple2(tag("/"), u32).read(line);
            Pipe { id: 1 << i, a, b }
        })
        .collect()
}

#[derive(Clone, Copy, Default)]
struct Bridge {
    len: usize,
    strength: u32,
    port: u32,
    used: u64,
}

impl Bridge {
    fn fuse(mut self, p: Pipe) -> Self {
        self.len += 1;
        self.strength += p.a + p.b;
        self.port = p.a + p.b - self.port;
        self.used |= p.id;
        self
    }

    fn build<T: Ord>(
        mut self,
        key: fn(&Bridge) -> T,
        neighbs: &[Vec<Pipe>],
        unused_singles: &mut [usize],
        visited: &mut AHashSet<u64>,
    ) -> Bridge {
        if !visited.insert(self.used) {
            return self;
        }
        let port = self.port as usize;
        let mut single_idx = None;
        if unused_singles[port] > 0 {
            single_idx = Some(port);
            unused_singles[port] -= 1;
            self.len += 1;
            self.strength += 2 * self.port;
        }
        self = neighbs[port]
            .iter()
            .filter(|&p| self.used & p.id == 0)
            .map(|&p| self.fuse(p).build(key, neighbs, unused_singles, visited))
            .fold(self, |a, b| max_by_key(a, b, key));
        if let Some(i) = single_idx {
            unused_singles[i] += 1;
        }
        self
    }
}

fn solve<T: Ord>(input: &str, key: fn(&Bridge) -> T) -> u32 {
    let pipes = parse_pipes(input);
    let mx = pipes.iter().flat_map(|p| [p.a, p.b]).max().unwrap() as usize;
    let mut neighbs = vec![vec![]; mx + 1];
    let mut singles = vec![0; mx + 1];
    for &pipe in &pipes {
        if pipe.a != pipe.b {
            neighbs[pipe.a as usize].push(pipe);
            neighbs[pipe.b as usize].push(pipe);
        } else {
            singles[pipe.a as usize] += 1;
        }
    }
    Bridge::default()
        .build(key, &neighbs, &mut singles, &mut AHashSet::new())
        .strength
}

pub fn part1(input: &str) -> u32 {
    solve(input, |b| b.strength)
}

pub fn part2(input: &str) -> u32 {
    solve(input, |b| (b.len, b.strength))
}
