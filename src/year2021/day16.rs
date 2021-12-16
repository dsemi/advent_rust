fn take<'a>(bits: &mut &'a [u64], n: usize) -> &'a [u64] {
    let r = &bits[..n];
    *bits = &bits[n..];
    r
}

fn bin(ds: &[u64]) -> u64 {
    ds.iter().fold(0, |a, b| a << 1 | b)
}

fn packet(bs: &mut &[u64]) -> (u64, u64) {
    let mut version = bin(take(bs, 3));
    let type_id = bin(take(bs, 3));
    if type_id == 4 {
        let mut n = 0;
        loop {
            let ds = take(bs, 5);
            n = n << 4 | bin(&ds[1..]);
            if ds[0] == 0 {
                return (version, n);
            }
        }
    }
    let mut ns = vec![];
    if take(bs, 1)[0] == 0 {
        let n = bin(take(bs, 15)) as usize;
        let mut r = take(bs, n);
        while !r.is_empty() {
            let (v, a) = packet(&mut r);
            version += v;
            ns.push(a);
        }
    } else {
        for _ in 0..bin(take(bs, 11)) {
            let (v, a) = packet(bs);
            version += v;
            ns.push(a);
        }
    }
    let n = match type_id {
        0 => ns.into_iter().sum(),
        1 => ns.into_iter().product(),
        2 => ns.into_iter().min().unwrap(),
        3 => ns.into_iter().max().unwrap(),
        5 => (ns[0] > ns[1]) as u64,
        6 => (ns[0] < ns[1]) as u64,
        7 => (ns[0] == ns[1]) as u64,
        _ => panic!("Bad type id: {}", type_id),
    };
    (version, n)
}

fn solve(input: &str) -> (u64, u64) {
    let bits = input
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap() as u64;
            vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
        })
        .collect::<Vec<_>>();
    packet(&mut &bits[..])
}

pub fn part1(input: &str) -> u64 {
    solve(input).0
}

pub fn part2(input: &str) -> u64 {
    solve(input).1
}
