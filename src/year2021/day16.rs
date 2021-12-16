fn take<'a>(n: usize, bits: &mut &'a [u64]) -> &'a [u64] {
    &std::mem::replace(bits, &bits[n..])[..n]
}

fn bin(ds: &[u64]) -> u64 {
    ds.iter().fold(0, |a, b| a << 1 | b)
}

fn packet(bs: &mut &[u64], vsum: &mut u64) -> u64 {
    *vsum += bin(take(3, bs));
    let type_id = bin(take(3, bs));
    if type_id == 4 {
        let mut n = 0;
        while take(1, bs)[0] == 1 {
            n = n << 4 | bin(take(4, bs));
        }
        return n << 4 | bin(take(4, bs));
    }
    let mut ns = vec![];
    if take(1, bs)[0] == 0 {
        let mut r = take(bin(take(15, bs)) as usize, bs);
        while !r.is_empty() {
            ns.push(packet(&mut r, vsum));
        }
    } else {
        (0..bin(take(11, bs))).for_each(|_| ns.push(packet(bs, vsum)));
    }
    match type_id {
        0 => ns.into_iter().sum(),
        1 => ns.into_iter().product(),
        2 => ns.into_iter().min().unwrap(),
        3 => ns.into_iter().max().unwrap(),
        5 => (ns[0] > ns[1]) as u64,
        6 => (ns[0] < ns[1]) as u64,
        7 => (ns[0] == ns[1]) as u64,
        _ => panic!("Bad type id: {}", type_id),
    }
}

fn solve(input: &str) -> (u64, u64) {
    let bits = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u64)
        .flat_map(|n| vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]);
    let mut version = 0;
    let n = packet(&mut &bits.collect::<Vec<_>>()[..], &mut version);
    (version, n)
}

pub fn part1(input: &str) -> u64 {
    solve(input).0
}

pub fn part2(input: &str) -> u64 {
    solve(input).1
}
