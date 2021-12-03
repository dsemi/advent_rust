fn most_freq_bit(ns: &[&[u8]], i: usize) -> u8 {
    (ns.iter().filter(|n| n[i] == b'1').count() >= (ns.len() + 1) / 2) as u8
}

pub fn part1(ns: Vec<&[u8]>) -> u32 {
    let gamma = (0..ns[0].len())
        .map(|i| most_freq_bit(&ns, i) as u32)
        .fold(0, |a, b| a << 1 | b);
    gamma * ((1 << ns[0].len()) - 1 ^ gamma)
}

fn most_matched(ns: &[&[u8]], pred: fn(u8, u8) -> bool) -> u32 {
    let mut ns = ns.to_owned();
    for i in 0..ns[0].len() {
        let c = most_freq_bit(&ns, i) + b'0';
        if ns.len() > 1 {
            ns.retain(|n| pred(n[i], c));
        }
    }
    ns[0].iter().fold(0, |a, &b| a << 1 | (b - b'0') as u32)
}

pub fn part2(ns: Vec<&[u8]>) -> u32 {
    most_matched(&ns, |a, b| a == b) * most_matched(&ns, |a, b| a != b)
}
