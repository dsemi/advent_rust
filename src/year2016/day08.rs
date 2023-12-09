use crate::utils::ocr::*;
use crate::utils::parsers::*;
use ahash::AHashSet;
use Instr::*;

const W: usize = 50;
const H: usize = 6;

enum Instr {
    Rect((usize, usize)),
    RotateRow((usize, usize)),
    RotateCol((usize, usize)),
}

fn parse(i: &mut &str) -> PResult<Instr> {
    alt((
        preceded("rect ", sep_tuple2(usize, 'x')).map(Rect),
        preceded("rotate row y=", sep_tuple2(usize, " by ")).map(RotateRow),
        preceded("rotate column x=", sep_tuple2(usize, " by ")).map(RotateCol),
    ))
    .parse_next(i)
}

fn process_instr(grid: &mut AHashSet<(usize, usize)>, instr: Instr) {
    match instr {
        Rect((a, b)) => {
            for c in 0..a {
                for r in 0..b {
                    grid.insert((r, c));
                }
            }
        }
        RotateRow((a, b)) => {
            *grid = grid
                .iter()
                .map(|(r, c)| (*r, if *r == a { (c + b) % W } else { *c }))
                .collect();
        }
        RotateCol((a, b)) => {
            *grid = grid
                .iter()
                .map(|(r, c)| (if *c == a { (r + b) % H } else { *r }, *c))
                .collect();
        }
    }
}

fn lit_pixels(input: &str) -> AHashSet<(usize, usize)> {
    let mut result = AHashSet::new();
    lines_iter(input, parse).for_each(|instr| process_instr(&mut result, instr));
    result
}

pub fn part1(input: &str) -> usize {
    lit_pixels(input).len()
}

pub fn part2(input: &str) -> String {
    let pix = lit_pixels(input);
    let mut display = vec!["".to_string()];
    for r in 0..H {
        display.push(
            (0..W)
                .map(|c| if pix.contains(&(r, c)) { '#' } else { ' ' })
                .collect(),
        );
    }
    parse_letters(&display.join("\n"), None)
}
