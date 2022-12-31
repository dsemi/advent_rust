use std::cmp::{max, min};

const SZ: i32 = 2500;

const DIRS: &[i32] = &[-SZ - 1, -SZ, -SZ + 1, -1, 1, SZ - 1, SZ, SZ + 1];

const PROP_DIRS: [[i32; 3]; 4] = [
    [-SZ - 1, -SZ, -SZ + 1],
    [SZ - 1, SZ, SZ + 1],
    [-SZ - 1, -1, SZ - 1],
    [-SZ + 1, 1, SZ + 1],
];

struct Elf {
    pos: i32,
    prop: Option<i32>,
}

fn parse(input: &str) -> (Vec<Elf>, Vec<Option<usize>>, Vec<i32>) {
    let elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, v)| {
                (v == '#').then(|| Elf {
                    pos: (y as i32 + SZ / 2) * SZ + x as i32 + SZ / 2,
                    prop: None,
                })
            })
        })
        .collect::<Vec<_>>();
    let mut grid = vec![None; SZ as usize * SZ as usize];
    for (idx, elf) in elves.iter().enumerate() {
        grid[elf.pos as usize] = Some(idx);
    }
    (elves, grid, vec![0; SZ as usize * SZ as usize])
}

fn step(elves: &mut [Elf], grid: &mut [Option<usize>], props: &mut [i32], dir: usize) -> bool {
    for elf in elves.iter_mut() {
        if DIRS.iter().any(|d| grid[(elf.pos + d) as usize].is_some()) {
            for i in 0..4 {
                let prop = PROP_DIRS[(dir + i) % 4];
                if grid[(elf.pos + prop[0]) as usize].is_none()
                    && grid[(elf.pos + prop[1]) as usize].is_none()
                    && grid[(elf.pos + prop[2]) as usize].is_none()
                {
                    let prop = elf.pos + prop[1];
                    elf.prop = Some(prop);
                    props[prop as usize] += 1;
                    break;
                }
            }
        }
    }
    let mut moved = false;
    for (idx, elf) in elves.iter_mut().enumerate() {
        if let Some(prop) = elf.prop.take() {
            if props[prop as usize] == 1 {
                moved = true;
                grid[elf.pos as usize] = None;
                grid[prop as usize] = Some(idx);
                elf.pos = prop;
            }
            props[prop as usize] = 0;
        }
    }
    moved
}

pub fn part1(input: &str) -> usize {
    let (mut elves, mut grid, mut props) = parse(input);
    for i in 0..10 {
        step(&mut elves, &mut grid, &mut props, i % 4);
    }
    let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);
    for elf in elves.iter() {
        let (x, y) = (elf.pos % SZ, elf.pos / SZ);
        min_x = min(min_x, x as usize);
        min_y = min(min_y, y as usize);
        max_x = max(max_x, x as usize + 1);
        max_y = max(max_y, y as usize + 1);
    }
    (max_x - min_x) * (max_y - min_y) - elves.len()
}

pub fn part2(input: &str) -> usize {
    let (mut elves, mut grid, mut props) = parse(input);
    for i in 0.. {
        if !step(&mut elves, &mut grid, &mut props, i % 4) {
            return i + 1;
        }
    }
    unreachable!()
}
