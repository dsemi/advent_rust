use std::cmp::max;

struct Pipe {
    a: u32,
    b: u32,
    used: bool,
}

fn parse_pipes(input: &str) -> Vec<Pipe> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('/').unwrap();
            Pipe {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                used: false,
            }
        })
        .collect()
}

fn solve<T: Copy + Ord>(input: &str, mut start: T, step: fn(T, &Pipe) -> T) -> T {
    let mut pipes = parse_pipes(input);
    fn dfs<T: Copy + Ord>(
        pipes: &mut Vec<Pipe>,
        start: &mut T,
        step: fn(T, &Pipe) -> T,
        pins: u32,
        v: T,
    ) {
        *start = max(*start, v);
        for i in 0..pipes.len() {
            if !pipes[i].used && (pipes[i].a == pins || pipes[i].b == pins) {
                pipes[i].used = true;
                dfs(
                    pipes,
                    start,
                    step,
                    if pipes[i].a == pins {
                        pipes[i].b
                    } else {
                        pipes[i].a
                    },
                    step(v, &pipes[i]),
                );
                pipes[i].used = false;
            }
        }
    }
    let v = start.clone();
    dfs(&mut pipes, &mut start, step, 0, v);
    start
}

pub fn part1(input: &str) -> u32 {
    solve(input, 0, |s, p| s + p.a + p.b)
}

pub fn part2(input: &str) -> u32 {
    solve(input, (0, 0), |s, p| (s.0 + 1, s.1 + p.a + p.b)).1
}
