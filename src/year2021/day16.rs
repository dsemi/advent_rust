struct Bits {
    bits: Vec<u64>,
    pos: usize,
}

impl Bits {
    fn take(&mut self, n: usize) -> &[u64] {
        self.pos += n;
        &self.bits[self.pos - n..self.pos]
    }
}

fn bin(ds: &[u64]) -> u64 {
    ds.iter().fold(0, |a, b| a << 1 | b)
}

fn packet(bs: &mut Bits) -> (u64, u64) {
    let mut version = bin(bs.take(3));
    let type_id = bin(bs.take(3));
    if type_id == 4 {
        let mut n = 0;
        loop {
            let ds = bs.take(5);
            n = n << 4 | bin(&ds[1..]);
            if ds[0] == 0 {
                return (version, n);
            }
        }
    }
    let mut ns = vec![];
    if bs.take(1)[0] == 0 {
        let n = bin(bs.take(15)) as usize + bs.pos;
        while bs.pos < n {
            let (v, a) = packet(bs);
            version += v;
            ns.push(a);
        }
    } else {
        for _ in 0..bin(bs.take(11)) {
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
    packet(&mut Bits {
        bits: input
            .chars()
            .flat_map(|c| {
                let n = c.to_digit(16).unwrap() as u64;
                vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
            })
            .collect(),
        pos: 0,
    })
}

pub fn part1(input: &str) -> u64 {
    solve(input).0
}

pub fn part2(input: &str) -> u64 {
    solve(input).1
}
