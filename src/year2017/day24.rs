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

fn build<T: Copy + Ord>(
    used: &mut Vec<bool>,
    a_s: &[Vec<&Pipe>],
    b_s: &[Vec<&Pipe>],
    step: fn(T, &Pipe) -> T,
    port: u32,
    curr: T,
) -> T {
    let mut mx = curr;
    for pipe in a_s[port as usize].iter() {
        if !used[pipe.id] {
            used[pipe.id] = true;
            mx = max(mx, build(used, a_s, b_s, step, pipe.b, step(curr, pipe)));
            used[pipe.id] = false;
        }
    }

    for pipe in b_s[port as usize].iter() {
        if !used[pipe.id] {
            used[pipe.id] = true;
            mx = max(mx, build(used, a_s, b_s, step, pipe.a, step(curr, pipe)));
            used[pipe.id] = false;
        }
    }
    mx
}

fn solve<T: Copy + Ord>(input: &str, start: T, step: fn(T, &Pipe) -> T) -> T {
    let pipes = parse_pipes(input);
    let mut used = vec![false; pipes.len()];
    let mx = pipes.iter().flat_map(|p| vec![p.a, p.b]).max().unwrap() as usize;
    let mut a_s: Vec<Vec<&Pipe>> = vec![vec![]; mx + 1];
    let mut b_s: Vec<Vec<&Pipe>> = vec![vec![]; mx + 1];
    for pipe in pipes.iter() {
        a_s[pipe.a as usize].push(pipe);
        if pipe.a != pipe.b {
            b_s[pipe.b as usize].push(pipe);
        }
    }
    build(&mut used, &a_s, &b_s, step, 0, start)
}

pub fn part1(input: &str) -> u32 {
    solve(input, 0, |s, p| s + p.a + p.b)
}

pub fn part2(input: &str) -> u32 {
    solve(input, (0, 0), |s, p| (s.0 + 1, s.1 + p.a + p.b)).1
}
