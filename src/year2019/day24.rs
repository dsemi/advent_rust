use hashbrown::HashSet;

fn parse(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|row| row.chars().map(|v| (v == '#') as u64))
        .rev()
        .fold(0, |a, b| a << 2 | b)
}

fn sacc(x: u64, a: u64) -> u64 {
    const MASK: u64 = 0x5555555555555555;
    ((x & !MASK) | ((x & MASK) + a)) ^ (a & x & (x >> 1))
}

fn even_bits(grid: u64) -> u32 {
    let mut b = (grid | (grid >> 31)) as u32;
    b = (b & 0x99999999) | (b & 0x22222222) << 1 | (b & 0x44444444) >> 1;
    b = (b & 0xc3c3c3c3) | (b & 0x0c0c0c0c) << 2 | (b & 0x30303030) >> 2;
    b = (b & 0xf00ff00f) | (b & 0x00f000f0) << 4 | (b & 0x0f000f00) >> 4;
    b = (b & 0xff0000ff) | (b & 0x0000ff00) << 8 | (b & 0x00ff0000) >> 8;
    b
}

fn neighbors4(grid: u64) -> u64 {
    let mut n = sacc(grid << 10, grid >> 10);
    n = sacc(n, (grid & 0x0ff3fcff3fcff) << 2);
    n = sacc(n, (grid & 0x3fcff3fcff3fc) >> 2);
    n
}

fn life_or_death(grid: u64, n: u64, mask: u64) -> u64 {
    let survived = grid & (n & !(n >> 1));
    let born = !grid & (n ^ (n >> 1));
    (survived | born) & mask
}

fn next(grid: u64) -> u64 {
    life_or_death(grid, neighbors4(grid), 0x1555555555555)
}

pub fn part1(input: &str) -> u32 {
    let mut planet = parse(input);
    let mut s = HashSet::new();
    while s.insert(planet) {
        planet = next(planet)
    }
    even_bits(planet)
}

fn step(inner: u64, grid: u64, outer: u64) -> u64 {
    const UMASK: u64 = 0x155;
    const DMASK: u64 = UMASK << 40;
    const LMASK: u64 = 0x10040100401;
    const RMASK: u64 = LMASK << 8;

    const IMASK: u64 = 0x404404000;
    const IUDMASK: u64 = 0x400004000;

    let oud =
        (u64::MAX - ((outer >> 14) & 1) + 1) & UMASK | (u64::MAX - ((outer >> 34) & 1) + 1) & DMASK;
    let olr =
        (u64::MAX - ((outer >> 22) & 1) + 1) & LMASK | (u64::MAX - ((outer >> 26) & 1) + 1) & RMASK;

    let iud = (inner & UMASK) << 10 | (inner & DMASK) >> 10;
    let ilr = (inner & LMASK) << 2 | (inner & RMASK) >> 2;

    let mut n = neighbors4(grid);
    n = sacc(n, oud);
    n = sacc(n, olr);
    n = sacc(n, (iud | ilr) & IMASK);
    n = sacc(n, (iud >> 2 | ilr >> 10) & IMASK);
    n = sacc(n, (iud << 2 | ilr << 10) & IMASK);
    n = sacc(n, ((iud >> 4 & IUDMASK) | ilr >> 20) & IMASK);
    n = sacc(n, ((iud << 4 & IUDMASK) | ilr << 20) & IMASK);

    life_or_death(grid, n, 0x1555554555555)
}

pub fn part2(input: &str) -> u32 {
    let mut planets = vec![0, parse(input), 0];
    for _ in 0..200 {
        planets = (0..planets.len() + 2)
            .map(|i| {
                step(
                    *planets.get(i - 2).unwrap_or(&0),
                    *planets.get(i - 1).unwrap_or(&0),
                    *planets.get(i).unwrap_or(&0),
                )
            })
            .collect();
    }
    planets.into_iter().map(|p| p.count_ones()).sum()
}
