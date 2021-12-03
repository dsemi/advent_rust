use counter::Counter;

fn count_bits(ns: &[&str], pos: usize, least: bool) -> u8 {
    ns.iter()
        .map(|n| n.as_bytes()[pos])
        .collect::<Counter<_>>()
        .most_common_tiebreaker(|&a, &b| b.cmp(&a))[least as usize]
        .0
}

fn bit_freqs(ns: &[&str], least: bool) -> i32 {
    i32::from_str_radix(
        &(0..ns[0].len())
            .map(|i| count_bits(&ns, i, least) as char)
            .collect::<String>(),
        2,
    )
    .unwrap()
}

pub fn part1(ns: Vec<&str>) -> i32 {
    bit_freqs(&ns, false) * bit_freqs(&ns, true)
}

fn most_matched(ns: &Vec<&str>, least: bool) -> Option<i32> {
    let mut ns = ns.clone();
    (0..ns[0].len())
        .filter_map(|i| {
            let c = count_bits(&ns, i, least);
            ns.retain(|n| n.as_bytes()[i] == c);
            (ns.len() == 1).then(|| i32::from_str_radix(&ns[0], 2).unwrap())
        })
        .next()
}

pub fn part2(ns: Vec<&str>) -> Option<i32> {
    Some(most_matched(&ns, false)? * most_matched(&ns, true)?)
}
