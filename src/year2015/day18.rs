fn parse(input: &str) -> Vec<u128> {
    let mut res = Vec::with_capacity(100);
    for line in input.lines() {
        let mut curr = 0;
        for v in line.chars() {
            curr <<= 1;
            curr |= (v == '#') as u128;
        }
        res.push(curr);
    }
    res
}

fn step(lights: &mut Vec<u128>) {
    let mut prev: u128 = 0;
    for i in 0..lights.len() {
        let curr = lights[i] << 1;
        let next = *lights.get(i + 1).unwrap_or(&0) << 1;
        let mut mask = 0b111;
        let mut focus = 0b10;
        let mut new_curr = 0;
        for _ in 0..100 {
            let c = curr & focus;
            let adj = (prev & mask).count_ones()
                + (curr & (mask - c)).count_ones()
                + (next & mask).count_ones();
            new_curr <<= 1;
            new_curr |= (adj == 3 || c != 0 && adj == 2) as u128;
            mask <<= 1;
            focus <<= 1;
        }
        prev = std::mem::replace(&mut lights[i], new_curr) << 1;
    }
}

pub fn part1(input: &str) -> u32 {
    let mut lights = parse(input);
    for _ in 0..100 {
        step(&mut lights);
    }
    lights.into_iter().map(|x| x.count_ones()).sum()
}

pub fn part2(input: &str) -> u32 {
    const CORNERS: u128 = 1 << 99 | 1;
    let mut lights = parse(input);
    lights[0] |= CORNERS;
    lights[99] |= CORNERS;
    for _ in 0..100 {
        step(&mut lights);
        lights[0] |= CORNERS;
        lights[99] |= CORNERS;
    }
    lights.into_iter().map(|x| x.count_ones()).sum()
}
