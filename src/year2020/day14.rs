use crate::utils::parsers::*;
use hashbrown::HashMap;

struct Cmd {
    mask: Vec<(char, u64)>,
    r: u64,
    v: u64,
}

fn parse_cmds(s: &str) -> Vec<Cmd> {
    let mut mask = Vec::new();
    let mut res = Vec::new();
    for line in s.lines() {
        if line.starts_with("mask") {
            mask = line[7..].chars().zip((0..36).rev()).collect();
        } else {
            let (_, r, _, v) = ("mem[", u64, "] = ", u64).read(line);
            res.push(Cmd {
                mask: mask.clone(),
                r,
                v,
            });
        }
    }
    res
}

pub fn part1(input: &str) -> u64 {
    let cmds = parse_cmds(input);
    let mut m = HashMap::new();
    for Cmd { mask, r, v } in cmds {
        let mut v = v;
        for (c, i) in mask {
            match c {
                '1' => v |= 1 << i,
                '0' => v &= !(1 << i),
                _ => (),
            }
        }
        m.insert(r, v);
    }
    m.values().sum()
}

fn set_vals(m: &mut HashMap<u64, u64>, xs: &[(char, u64)], r: u64, v: u64) {
    if xs.is_empty() {
        m.insert(r, v);
        return;
    }
    let (c, i) = xs[0];
    match c {
        '1' => set_vals(m, &xs[1..], r | (1 << i), v),
        '0' => set_vals(m, &xs[1..], r, v),
        'X' => {
            set_vals(m, &xs[1..], r | (1 << i), v);
            set_vals(m, &xs[1..], r & !(1 << i), v);
        }
        _ => panic!("Invalid bit: {}", c),
    }
}

pub fn part2(input: &str) -> u64 {
    let cmds = parse_cmds(input);
    let mut m = HashMap::new();
    for Cmd { mask, r, v } in cmds {
        set_vals(&mut m, &mask[..], r, v);
    }
    m.values().sum()
}
