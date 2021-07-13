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

enum Instr {
    Draw((i64, i64), Tile),
    Score(i64),
}

fn parse_instrs<F>(mut prog: intcode::Program, mut process: F)
where
    F: FnMut(Instr) -> Option<i64>,
{
    let mut i = 0;
    let mut buf = [0; 3];
    while !prog.done {
        prog.run();
        for v in prog.output.drain(..) {
            buf[i] = v;
            i += 1;
            if i == 3 {
                i = 0;
                let v = match buf {
                    [-1, 0, score] => process(Instr::Score(score)),
                    [x, y, tile] => {
                        process(Instr::Draw((x, y), FromPrimitive::from_i64(tile).unwrap()))
                    }
                };
                if let Some(x) = v {
                    prog.input.push_back(x);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut result = 0;
    parse_instrs(intcode::new(input), |instr| {
        if let Instr::Draw(_, Tile::Block) = instr {
            result += 1;
        }
        None
    });
    result
}

pub fn part2(input: &str) -> i64 {
    let mut prog = intcode::new(input);
    prog[0] = 2;
    let mut ball = 0;
    let mut paddle = 0;
    let mut score = 0;
    parse_instrs(prog, |instr| {
        match instr {
            Instr::Draw((x, _), Tile::Ball) => {
                ball = x;
                return Some(ball.cmp(&paddle) as i64);
            }
            Instr::Draw((x, _), Tile::Paddle) => paddle = x,
            Instr::Score(v) => score = v,
            _ => (),
        }
        None
    });
    score
}
