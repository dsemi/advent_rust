use std::cmp::max;

fn run_commands(
    input: &str,
    off: fn(i32) -> i32,
    on: fn(i32) -> i32,
    toggle: fn(i32) -> i32,
) -> i32 {
    let mut grid = vec![0; 1000000];
    for line in input.lines() {
        let (cmdstr, x0, y0, x1, y1) = scan_fmt!(
            line,
            "{[^0-9]}{},{} through {},{}",
            String,
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
        let f = match &cmdstr[..] {
            "turn off " => off,
            "turn on " => on,
            "toggle " => toggle,
            _ => panic!("unknown action: {}", cmdstr),
        };

        for x in x0..=x1 {
            for y in y0..=y1 {
                let v = &mut grid[1000 * x + y];
                *v = f(*v);
            }
        }
    }
    grid.into_iter().sum()
}

pub fn part1(input: &str) -> i32 {
    run_commands(input, |_| 0, |_| 1, |x| x ^ 1)
}

pub fn part2(input: &str) -> i32 {
    run_commands(input, |x| max(0, x - 1), |x| x + 1, |x| x + 2)
}
