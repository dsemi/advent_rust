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
    let mut prev = 0;
    for i in 0..lights.len() {
        let mut curr = lights[i];
        let next = *lights.get(i + 1).unwrap_or(&0);
        let mut adjs = vec![prev << 1, prev, prev >> 1,
                            curr << 1,       curr >> 1,
                            next << 1, next, next >> 1];
        let mut curr2 = 0;
        for _ in 0..lights.len() {
            curr2 <<= 1;
            let mut adj = 0;
            for v in adjs.iter_mut() {
                adj += (*v & 1) as u8;
                *v >>= 1;
            }
            let mut c = curr & 1;
            curr >>= 1;
            if c > 0 && adj != 2 && adj != 3 {
                c = 0;
            } else if c == 0 && adj == 3 {
                c = 1;
            }
            curr2 |= c;
        }
        prev = std::mem::replace(&mut lights[i], curr2);
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
