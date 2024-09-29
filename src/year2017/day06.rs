const REMOVE: u128 = 0x00ffffffffffffffffffffffffffffff;
const REDIST: [u128; 17] = [
    0x00000000000000000000000000000000,
    0x00010000000000000000000000000000,
    0x00010100000000000000000000000000,
    0x00010101000000000000000000000000,
    0x00010101010000000000000000000000,
    0x00010101010100000000000000000000,
    0x00010101010101000000000000000000,
    0x00010101010101010000000000000000,
    0x00010101010101010100000000000000,
    0x00010101010101010101000000000000,
    0x00010101010101010101010000000000,
    0x00010101010101010101010100000000,
    0x00010101010101010101010101000000,
    0x00010101010101010101010101010000,
    0x00010101010101010101010101010100,
    0x00010101010101010101010101010101,
    0x01010101010101010101010101010101,
];
const MASK: u128 = 0x10101010101010101010101010101010;

fn init(ns: Vec<usize>) -> u128 {
    ns.into_iter().fold(0, |acc, n| (acc << 8) + n as u128)
}

fn next(mem: u128) -> u128 {
    let mut mask = MASK;
    for n in 0..5 {
        let m = (mem << n) & mask;
        mask = if m == 0 { mask } else { m };
    }
    let offset = mask.leading_zeros() - 3;
    let max = (mem.rotate_left(offset + 8) & 0xff) as usize;
    (mem & REMOVE.rotate_right(offset)) + REDIST[max].rotate_right(offset)
}

fn redistribute_until_cycle(ns: Vec<usize>) -> (usize, usize) {
    let mem = init(ns);
    let mut h0 = next(mem);
    let mut h1 = next(next(mem));
    while h0 != h1 {
        h0 = next(h0);
        h1 = next(next(h1));
    }
    let mut mu = 0;
    let mut h0 = mem;
    while h0 != h1 {
        h0 = next(h0);
        h1 = next(h1);
        mu += 1;
    }
    let mut lambda = 1;
    let mut h1 = next(h0);
    while h0 != h1 {
        h1 = next(h1);
        lambda += 1;
    }
    (mu + lambda, lambda)
}

pub fn part1(input: Vec<usize>) -> usize {
    redistribute_until_cycle(input).0
}

pub fn part2(input: Vec<usize>) -> usize {
    redistribute_until_cycle(input).1
}
