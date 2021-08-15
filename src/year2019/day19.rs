use crate::year2019::intcode;

fn is_pulled(prog: &intcode::Program, x: i64, y: i64) -> bool {
    let mut prog = prog.clone();
    prog.input.extend(&[x, y]);
    prog.run();
    prog.output.pop_front().unwrap() == 1
}

pub fn part1(input: &str) -> i32 {
    let prog = intcode::new(input);
    (0..50)
        .map(|x| (0..50).map(|y| is_pulled(&prog, x, y) as i32).sum::<i32>())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let prog = intcode::new(input);
    let (mut x, mut y) = (0, 0);
    while !is_pulled(&prog, x + 99, y) {
        if !is_pulled(&prog, x, y + 100) {
            x += 1;
        }
        y += 1;
    }
    x * 10000 + y
}
