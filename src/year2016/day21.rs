use crate::utils::parsers::*;
use Dir::*;
use Instr::*;

fn rot_chr_idx(i: usize) -> usize {
    if i >= 4 {
        i + 2
    } else {
        i + 1
    }
}

fn move_p<T>(s: &mut Vec<T>, i: usize, j: usize) {
    let c = s.remove(i);
    s.insert(j, c);
}

#[derive(Clone)]
enum Dir {
    L,
    R,
}

enum Instr {
    SwapPos((usize, usize)),
    SwapChr((char, char)),
    Rotate((Dir, usize)),
    RotatePos(char),
    Reverse((usize, usize)),
    Move((usize, usize)),
}

fn parse(i: &mut &str) -> PResult<Instr> {
    alt((
        (
            preceded("swap position ", usize),
            preceded(" with position ", usize),
        )
            .map(SwapPos),
        (
            preceded("swap letter ", any),
            preceded(" with letter ", any),
        )
            .map(SwapChr),
        (
            preceded("rotate ", alt(("left".value(L), "right".value(R)))),
            delimited(' ', usize, (' ', alpha1)),
        )
            .map(Rotate),
        preceded("rotate based on position of letter ", any).map(RotatePos),
        (
            preceded("reverse positions ", usize),
            preceded(" through ", usize),
        )
            .map(Reverse),
        (
            preceded("move position ", usize),
            preceded(" to position ", usize),
        )
            .map(Move),
    ))
    .parse_next(i)
}

fn run_program<'a, I: Iterator<Item = &'a str>>(input: String, instrs: I, invert: bool) -> String {
    let mut mem: Vec<char> = input.chars().collect();
    instrs.map(|i| parse.read(i)).for_each(|instr| match instr {
        SwapPos((x, y)) => {
            mem.swap(x, y);
        }
        SwapChr((a, b)) => {
            let x = mem.iter().position(|x| *x == a).unwrap();
            let y = mem.iter().position(|x| *x == b).unwrap();
            mem.swap(x, y);
        }
        Rotate((d, x)) => match (d, invert) {
            (L, false) | (R, true) => {
                mem.rotate_left(x);
            }
            (L, true) | (R, false) => {
                mem.rotate_right(x);
            }
        },
        RotatePos(c) => {
            if invert {
                for i in 0.. {
                    if rot_chr_idx(mem.iter().position(|x| *x == c).unwrap()) == i {
                        break;
                    }
                    mem.rotate_left(1);
                }
            } else {
                let i = (mem.len() - rot_chr_idx(mem.iter().position(|x| *x == c).unwrap()))
                    .rem_euclid(mem.len());
                mem.rotate_left(i);
            }
        }
        Reverse((x, y)) => {
            mem[x..=y].reverse();
        }
        Move((x, y)) => {
            let (i, j) = if invert { (y, x) } else { (x, y) };
            move_p(&mut mem, i, j);
        }
    });
    mem.into_iter().collect()
}

pub fn part1(input: &str) -> String {
    run_program("abcdefgh".to_owned(), input.lines(), false)
}

pub fn part2(input: &str) -> String {
    run_program("fbgdceah".to_owned(), input.lines().rev(), true)
}
