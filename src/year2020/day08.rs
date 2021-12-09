use ahash::AHashSet;
use Instr::*;

#[derive(Clone, Copy)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_prog(s: &str) -> Vec<Instr> {
    s.lines()
        .map(|line| {
            let w: Vec<&str> = line.split(' ').collect();
            match w[0] {
                "acc" => Acc(w[1].parse().unwrap()),
                "jmp" => Jmp(w[1].parse().unwrap()),
                "nop" => Nop(w[1].parse().unwrap()),
                _ => panic!("Invalid instr: {}", w[0]),
            }
        })
        .collect()
}

fn run_prog(prog: &[Instr]) -> (i64, bool) {
    let mut visited = AHashSet::new();
    let mut acc = 0;
    let mut i = 0;
    while 0 <= i && i < prog.len() as i64 {
        if !visited.insert(i) {
            return (acc, false);
        }
        match prog[i as usize] {
            Acc(n) => acc += n,
            Jmp(n) => i += n - 1,
            _ => (),
        }
        i += 1;
    }
    (acc, true)
}

pub fn part1(input: &str) -> i64 {
    run_prog(&parse_prog(input)).0
}

fn flip(prog: &mut [Instr], i: usize) {
    prog[i] = match prog[i] {
        Jmp(n) => Nop(n),
        Nop(n) => Jmp(n),
        x => x,
    };
}

pub fn part2(input: &str) -> i64 {
    let mut prog = parse_prog(input);
    for i in 0..prog.len() {
        flip(&mut prog, i);
        let (ans, fin) = run_prog(&prog);
        flip(&mut prog, i);
        if fin {
            return ans;
        }
    }
    0
}
