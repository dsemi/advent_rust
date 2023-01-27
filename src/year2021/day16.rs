use itertools::unfold;

fn bin(bs: &mut dyn Iterator<Item = u64>, n: usize) -> u64 {
    bs.take(n).fold(0, |a, b| a << 1 | b)
}

fn packet(bs: &mut dyn Iterator<Item = u64>, vsum: &mut u64) -> u64 {
    *vsum += bin(bs, 3);
    let type_id = bin(bs, 3);
    if type_id == 4 {
        let mut n = 0;
        while let Some(1) = bs.next() {
            n = n << 4 | bin(bs, 4);
        }
        return n << 4 | bin(bs, 4);
    }
    let ns: Vec<u64> = if let Some(0) = bs.next() {
        let n = bin(bs, 15);
        let r = bs.take(n as usize).peekable();
        unfold(r, |mut r| r.peek().is_some().then(|| packet(&mut r, vsum))).collect()
    } else {
        (0..bin(bs, 11)).map(|_| packet(bs, vsum)).collect()
    };
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
    let mut bits = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u64)
        .flat_map(|n| vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]);
    let mut version = 0;
    let n = packet(&mut bits, &mut version);
    (version, n)
}

pub fn part1(input: &str) -> u64 {
    solve(input).0
}

pub fn part2(input: &str) -> u64 {
    solve(input).1
}
