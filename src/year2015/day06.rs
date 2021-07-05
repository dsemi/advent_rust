use regex::Regex;
use std::cmp::max;

fn run_commands(
    input: &str,
    off: fn(i32) -> i32,
    on: fn(i32) -> i32,
    toggle: fn(i32) -> i32,
) -> i32 {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = vec![0; 1000000];
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let (cmdstr, x0, y0, x1, y1) = (&cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
        let f = match cmdstr {
            "turn off" => off,
            "turn on" => on,
            "toggle" => toggle,
            _ => panic!("unknown action: {}", cmdstr),
        };
        let (x0, y0, x1, y1): (usize, usize, usize, usize) = (
            x0.parse().unwrap(),
            y0.parse().unwrap(),
            x1.parse().unwrap(),
            y1.parse().unwrap(),
        );

        for x in x0..=x1 {
            for y in y0..=y1 {
                grid[1000 * x + y] = f(grid[1000 * x + y]);
            }
        }
    }
    grid.iter().sum()
}

pub fn part1(input: &str) -> i32 {
    run_commands(input, |_| 0, |_| 1, |x| x ^ 1)
}

pub fn part2(input: &str) -> i32 {
    run_commands(input, |x| max(0, x - 1), |x| x + 1, |x| x + 2)
}
