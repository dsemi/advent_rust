use std::cell::RefCell;
use std::cmp::max;

struct Pipe {
    id: usize,
    a: u32,
    b: u32,
}

fn parse_pipes(input: &str) -> Vec<Pipe> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (a, b) = line.split_once('/').unwrap();
            Pipe {
                id,
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
            }
        })
        .collect()
}

struct Builder<'a, T> {
    used: RefCell<Vec<bool>>,
    a_s: Vec<Vec<&'a Pipe>>,
    b_s: Vec<Vec<&'a Pipe>>,
    f: fn(T, &Pipe) -> T,
}

impl<'a, T: Copy + Ord> Builder<'a, T> {
    fn build(&self, port: u32, curr: T) -> T {
        let mut m = curr;
        for vec in [&self.a_s, &self.b_s] {
            for pipe in vec[port as usize].iter() {
                if !self.used.borrow()[pipe.id] {
                    self.used.borrow_mut()[pipe.id] = true;
                    m = max(m, self.build(pipe.a + pipe.b - port, (self.f)(curr, pipe)));
                    self.used.borrow_mut()[pipe.id] = false;
                }
            }
        }
        m
    }
}

fn solve<T: Copy + Ord>(input: &str, start: T, step: fn(T, &Pipe) -> T) -> T {
    let pipes = parse_pipes(input);
    let used = vec![false; pipes.len()];
    let mx = pipes.iter().flat_map(|p| vec![p.a, p.b]).max().unwrap() as usize;
    let mut a_s: Vec<Vec<&Pipe>> = vec![vec![]; mx + 1];
    let mut b_s: Vec<Vec<&Pipe>> = vec![vec![]; mx + 1];
    for pipe in pipes.iter() {
        a_s[pipe.a as usize].push(pipe);
        if pipe.a != pipe.b {
            b_s[pipe.b as usize].push(pipe);
        }
    }
    Builder {
        a_s,
        b_s,
        used: RefCell::new(used),
        f: step,
    }
    .build(0, start)
}

pub fn part1(input: &str) -> u32 {
    solve(input, 0, |s, p| s + p.a + p.b)
}

pub fn part2(input: &str) -> u32 {
    solve(input, (0, 0), |s, p| (s.0 + 1, s.1 + p.a + p.b)).1
}
