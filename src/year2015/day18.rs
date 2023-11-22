const ROWS: usize = 100;
const COLS: usize = 100;
const BITS_PER_CELL: usize = 4;

type Cluster = u16;
const CLUSTERS: usize = COLS * BITS_PER_CELL / Cluster::BITS as usize;

type Lights = [[Cluster; CLUSTERS]; ROWS];

fn parse(input: &str) -> Lights {
    let mut res = [[0; CLUSTERS]; ROWS];
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            let idx = x / (Cluster::BITS as usize / BITS_PER_CELL);
            res[y][idx] <<= BITS_PER_CELL;
            res[y][idx] |= (v == '#') as Cluster;
        }
    }
    res
}

fn step(lights: &mut Lights) {
    const STEP_MASK: u16 = {
        let mut mask = 0;
        let mut n = 0;
        while n < Cluster::BITS as usize / BITS_PER_CELL {
            mask <<= 4;
            mask |= 1;
            n += 1;
        }
        mask
    };
    let mut temp = [[0; CLUSTERS]; ROWS];
    for y in 0..ROWS {
        for x in 0..CLUSTERS {
            let mut sum =
                lights[y][x] + (lights[y][x] >> BITS_PER_CELL) + (lights[y][x] << BITS_PER_CELL);
            if x > 0 {
                sum += lights[y][x - 1] << (Cluster::BITS - BITS_PER_CELL as u32);
            }
            if x < CLUSTERS - 1 {
                sum += lights[y][x + 1] >> (Cluster::BITS - BITS_PER_CELL as u32);
            }
            temp[y][x] = sum;
        }
    }
    for y in 0..ROWS {
        for x in 0..CLUSTERS {
            let mut sum = temp[y][x] - lights[y][x];
            if y > 0 {
                sum += temp[y - 1][x];
            }
            if y < ROWS - 1 {
                sum += temp[y + 1][x];
            }
            let a = sum >> 3;
            let b = sum >> 2;
            let c = sum >> 1;
            let d = sum | lights[y][x];
            lights[y][x] = (!a & !b & c & d) & STEP_MASK;
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut lights = parse(input);
    for _ in 0..100 {
        step(&mut lights);
    }
    lights
        .into_iter()
        .map(|row| row.into_iter().map(|x| x.count_ones()).sum::<u32>())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut lights = parse(input);
    lights[0][0] |= 1 << (Cluster::BITS - BITS_PER_CELL as u32);
    lights[0][CLUSTERS - 1] |= 1;
    lights[ROWS - 1][0] |= 1 << (Cluster::BITS - BITS_PER_CELL as u32);
    lights[ROWS - 1][CLUSTERS - 1] |= 1;
    for _ in 0..100 {
        step(&mut lights);
        lights[0][0] |= 1 << (Cluster::BITS - BITS_PER_CELL as u32);
        lights[0][CLUSTERS - 1] |= 1;
        lights[ROWS - 1][0] |= 1 << (Cluster::BITS - BITS_PER_CELL as u32);
        lights[ROWS - 1][CLUSTERS - 1] |= 1;
    }
    lights
        .into_iter()
        .map(|row| row.into_iter().map(|x| x.count_ones()).sum::<u32>())
        .sum()
}
