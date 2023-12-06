use crate::utils::parsers::*;
use ahash::AHashMap;

fn run(mut r: AHashMap<&str, i64>, input: &str) -> i64 {
    let s = input.replace(',', "");
    let instrs: Vec<Vec<&str>> = s.lines().map(|line| line.split(' ').collect()).collect();
    let mut i: i64 = 0;
    while i >= 0 && (i as usize) < instrs.len() {
        let instr = &instrs[i as usize];
        match instr[0] {
            "hlf" => *r.get_mut(instr[1]).unwrap() /= 2,
            "tpl" => *r.get_mut(instr[1]).unwrap() *= 3,
            "inc" => *r.get_mut(instr[1]).unwrap() += 1,
            "jmp" => i += instr[1].int::<i64>() - 1,
            "jie" => {
                if r[instr[1]] % 2 == 0 {
                    i += instr[2].int::<i64>() - 1;
                }
            }
            "jio" => {
                if r[instr[1]] == 1 {
                    i += instr[2].int::<i64>() - 1;
                }
            }
            _ => panic!("Bad instr"),
        }
        i += 1;
    }
    r["b"]
}

pub fn part1(input: &str) -> i64 {
    run(vec![("a", 0), ("b", 0)].into_iter().collect(), input)
}

pub fn part2(input: &str) -> i64 {
    run(vec![("a", 1), ("b", 0)].into_iter().collect(), input)
}
