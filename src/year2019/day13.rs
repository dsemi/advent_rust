use genawaiter::rc::{Co, Gen};
use genawaiter::GeneratorState;
use num_traits::FromPrimitive;

use crate::year2019::intcode;

#[derive(FromPrimitive)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

struct Draw((i64, i64), Tile);

async fn run(mut prog: intcode::Program, co: Co<Draw, Option<i64>>) -> i64 {
    let mut i = 0;
    let mut buf = [0; 3];
    let mut score = 0;
    while !prog.done {
        prog.run();
        for v in prog.output.drain(..) {
            buf[i] = v;
            i += 1;
            if i == 3 {
                i = 0;
                match buf {
                    [-1, 0, scr] => score = scr,
                    [x, y, tile] => {
                        let tile = FromPrimitive::from_i64(tile).unwrap();
                        if let Some(x) = co.yield_(Draw((x, y), tile)).await {
                            prog.input.push_back(x);
                        }
                    }
                }
            }
        }
    }
    score
}

pub fn part1(input: &str) -> usize {
    let mut result = 0;
    let mut gen = Gen::new(|co| run(intcode::new(input), co));
    while let GeneratorState::Yielded(instr) = gen.resume_with(None) {
        result += matches!(instr, Draw(_, Tile::Block)) as usize;
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut prog = intcode::new(input);
    prog[0] = 2;
    let mut paddle_x = 0;
    let mut inp = None;
    let mut gen = Gen::new(|co| run(prog, co));
    loop {
        match gen.resume_with(inp.take()) {
            GeneratorState::Yielded(Draw((ball_x, _), Tile::Ball)) => {
                inp = Some(ball_x.cmp(&paddle_x) as i64)
            }
            GeneratorState::Yielded(Draw((x, _), Tile::Paddle)) => paddle_x = x,
            GeneratorState::Complete(score) => return score,
            _ => (),
        }
    }
}
