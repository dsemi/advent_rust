use crate::utils::parsers::*;
use advent::Parser;
use Instr::*;

#[derive(Clone, Copy, Parser)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn run_prog(prog: &[Instr]) -> (i64, bool) {
    let mut vis = vec![false; prog.len()];
    let mut acc = 0;
    let mut i = 0;
    while 0 <= i && i < prog.len() as i64 && !std::mem::replace(&mut vis[i as usize], true) {
        match prog[i as usize] {
            Acc(n) => acc += n,
            Jmp(n) => i += n - 1,
            _ => (),
        }
        i += 1;
    }
    (acc, i == prog.len() as i64)
}

pub fn part1(input: &str) -> i64 {
    run_prog(&lines(instr).read(input)).0
}

fn flip(prog: &mut [Instr], i: usize) {
    prog[i] = match prog[i] {
        Jmp(n) => Nop(n),
        Nop(n) => Jmp(n),
        x => x,
    };
}

pub fn part2(input: &str) -> Option<i64> {
    let mut prog = lines(instr).read(input);
    (0..prog.len()).find_map(|i| {
        flip(&mut prog, i);
        let (ans, fin) = run_prog(&prog);
        flip(&mut prog, i);
        fin.then_some(ans)
    })
}
