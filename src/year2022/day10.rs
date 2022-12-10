use crate::utils::*;

fn run(input: &str) -> impl Iterator<Item = (usize, i32)> + '_ {
    input
        .split_whitespace()
        .map(|tok| match tok {
            "addx" | "noop" => 0,
            _ => tok.parse().unwrap(),
        })
        .good_scan(1, |x, d| x + d)
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .take(240)
}

pub fn part1(input: &str) -> i32 {
    run(input)
        .filter_map(|(cycle, x)| {
            [20, 60, 100, 140, 180, 220]
                .contains(&cycle)
                .then(|| cycle as i32 * x)
        })
        .sum()
}

pub fn part2(input: &str) -> String {
    let mut res = "\n".to_owned();
    for (cycle, x) in run(input) {
        res.push(if ((cycle as i32 - 1) % 40 - x).abs() <= 1 {
            '#'
        } else {
            ' '
        });
        if cycle < 240 && cycle % 40 == 0 {
            res.push('\n');
        }
    }
    res
}
