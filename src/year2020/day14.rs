use ahash::AHashMap;

fn parse_cmds(s: &str) -> Vec<(Vec<(u64, char)>, u64, u64)> {
    let mut mask = Vec::new();
    let mut res = Vec::new();
    for line in s.lines() {
        if line.starts_with("mask") {
            mask = line
                .split(" ")
                .last()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(i, x)| (35 - i as u64, x))
                .collect();
        } else {
            let (r, v) = scan_fmt!(line, "mem[{}] = {}", u64, u64).unwrap();
            res.push((mask.clone(), r, v));
        }
    }
    res
}

pub fn part1(input: &str) -> u64 {
    let cmds = parse_cmds(input);
    let mut m = AHashMap::new();
    for (mask, r, v) in cmds {
        let mut v = v;
        for (i, c) in mask {
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

fn set_vals(m: &mut AHashMap<u64, u64>, xs: &[(u64, char)], r: u64, v: u64) {
    if xs.is_empty() {
        m.insert(r, v);
        return;
    }
    let (i, c) = xs[0];
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
    let mut m = AHashMap::new();
    for (mask, r, v) in cmds {
        set_vals(&mut m, &mask[..], r, v);
    }
    m.values().sum()
}
