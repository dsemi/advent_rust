use ahash::AHashSet;
use scan_fmt::scan_fmt as scanf;
use std::cmp::max;

fn parse(input: &str) -> (AHashSet<(usize, usize)>, &str) {
    let (dots, instrs) = input.split_once("\n\n").unwrap();
    (
        dots.lines()
            .map(|dot| {
                let (x, y) = dot.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
        instrs,
    )
}

fn fold(paper: &mut AHashSet<(usize, usize)>, instr: &str) {
    let (d, n) = scanf!(instr, "fold along {}={}", char, usize).unwrap();
    if d == 'x' {
        let mut rights = AHashSet::new();
        paper.retain(|&(x, y)| {
            if x > n {
                rights.insert((2 * n - x, y));
                return false;
            }
            true
        });
        paper.extend(rights);
    } else {
        let mut bottoms = AHashSet::new();
        paper.retain(|&(x, y)| {
            if y > n {
                bottoms.insert((x, 2 * n - y));
                return false;
            }
            true
        });
        paper.extend(bottoms);
    }
}

pub fn part1(input: &str) -> usize {
    let (mut paper, instrs) = parse(input);
    fold(&mut paper, instrs.lines().next().unwrap());
    paper.len()
}

pub fn part2(input: &str) -> String {
    let (mut paper, instrs) = parse(input);
    for instr in instrs.lines() {
        fold(&mut paper, instr);
    }
    let (mx, my) = paper
        .iter()
        .fold((0, 0), |(mx, my), (x, y)| (max(mx, *x), max(my, *y)));
    let mut display = vec!["".to_owned()];
    for y in 0..=my {
        display.push(
            (0..=mx)
                .map(|x| if paper.contains(&(x, y)) { '#' } else { ' ' })
                .collect(),
        );
    }
    display.push("".to_owned());
    display.join("\n")
}
