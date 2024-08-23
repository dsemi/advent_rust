fn idx(b: u8) -> usize {
    if b >= b'a' {
        (b - b'a') as usize
    } else {
        (b - b'A') as usize + 26
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let half = line.len() / 2;
            let mut v = [0; 52];
            line[..half].bytes().for_each(|b| v[idx(b)] |= 1);
            line[half..].bytes().map(idx).find(|&i| v[i] > 0).unwrap() + 1
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .chunks(3)
        .map(|pts| {
            let mut v = [0; 52];
            pts[0].bytes().for_each(|b| v[idx(b)] |= 0b01);
            pts[1].bytes().for_each(|b| v[idx(b)] |= 0b10);
            pts[2].bytes().map(idx).find(|&i| v[i] == 0b11).unwrap() + 1
        })
        .sum()
}
