pub fn part1(ns: Vec<&[u8]>) -> u32 {
    let gamma = (0..ns[0].len())
        .map(|i| (ns.iter().filter(|n| n[i] == b'1').count() >= (ns.len() + 1) / 2) as u32)
        .fold(0, |a, b| (a << 1) | b);
    gamma * (((1 << ns[0].len()) - 1) ^ gamma)
}

fn most_matched(ns: &[&[u8]], pred: fn(usize, usize) -> bool) -> u32 {
    let mut ns = ns.to_owned();
    for i in 0..ns[0].len() {
        let (a, b) = ns.into_iter().partition::<Vec<_>, _>(|n| n[i] == b'1');
        ns = if pred(a.len(), b.len()) { a } else { b };
    }
    ns[0].iter().fold(0, |a, &b| (a << 1) | (b - b'0') as u32)
}

pub fn part2(ns: Vec<&[u8]>) -> u32 {
    most_matched(&ns, |a, b| a >= b) * most_matched(&ns, |a, b| a < b && a != 0 || b == 0)
}
