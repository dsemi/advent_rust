use super::assembunny::{Instr::*, Val::Lit, parse_instrs};

const MASK: i64 = 0xAAAAAAAA;

pub fn part1(input: &str) -> Option<i64> {
    let ssim = parse_instrs(input);
    match ssim.instrs[1..] {
        [Cpy(Lit(c), _), Mul(Lit(b), _, _, _), ..] => (0..).find(|i| {
            let v = i + b * c;
            v == MASK & (v.isolate_highest_one() * 2 - 1)
        }),
        _ => {
            eprintln!("slow path");
            (0..).find(|i| {
                let mut sim = ssim.clone();
                sim.regs[0] = *i;
                sim.take(10).zip([0, 1].iter().cycle()).all(|(a, b)| a == *b)
            })
        }
    }
}

pub fn part2(_: &str) -> String {
    let input: &str = include_str!("../../inputs/2016/bonuschallenge.txt").trim_end();
    let sim = parse_instrs(input);
    let output: String = sim.into_iter().map(|x| x as u8 as char).collect();
    super::day08::part2(&output)
}
