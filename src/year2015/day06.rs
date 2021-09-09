fn run_commands(
    input: &str,
    off: fn(&mut [u32]),
    on: fn(&mut [u32]),
    toggle: fn(&mut [u32]),
) -> u32 {
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
            let row = 1000 * x;
            f(&mut grid[row + y0..=row + y1]);
        }
    }
    grid.into_iter().sum()
}

pub fn part1(input: &str) -> u32 {
    run_commands(
        input,
        |s| s.fill(0),
        |s| s.fill(1),
        |s| s.iter_mut().for_each(|v| *v ^= 1),
    )
}

pub fn part2(input: &str) -> u32 {
    run_commands(
        input,
        |s| {
            s.iter_mut()
                .for_each(|v| *v = v.checked_sub(1).unwrap_or(0))
        },
        |s| s.iter_mut().for_each(|v| *v += 1),
        |s| s.iter_mut().for_each(|v| *v += 2),
    )
}
