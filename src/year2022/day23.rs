use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

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
    prop: i32,
}

#[allow(clippy::type_complexity)]
fn parse(
    input: &str,
) -> (
    Vec<Rc<RefCell<Elf>>>,
    Vec<Option<Rc<RefCell<Elf>>>>,
    Vec<i32>,
) {
    let elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, v)| {
                (v == '#').then(|| {
                    Rc::new(RefCell::new(Elf {
                        pos: (y as i32 + SZ / 2) * SZ + x as i32 + SZ / 2,
                        prop: i32::MIN,
                    }))
                })
            })
        })
        .collect::<Vec<_>>();
    let mut grid = vec![None; SZ as usize * SZ as usize];
    for elf in elves.iter() {
        grid[elf.borrow().pos as usize] = Some(elf.clone());
    }
    (elves, grid, vec![0; SZ as usize * SZ as usize])
}

fn step(
    elves: &[Rc<RefCell<Elf>>],
    grid: &mut [Option<Rc<RefCell<Elf>>>],
    props: &mut [i32],
    dir: usize,
) -> bool {
    for elf in elves {
        let pos = elf.borrow().pos;
        if DIRS.iter().any(|d| grid[(pos + d) as usize].is_some()) {
            for i in 0..4 {
                let prop = PROP_DIRS[(dir + i) % 4];
                if grid[(elf.borrow().pos + prop[0]) as usize].is_none()
                    && grid[(pos + prop[1]) as usize].is_none()
                    && grid[(pos + prop[2]) as usize].is_none()
                {
                    let prop = pos + prop[1];
                    elf.borrow_mut().prop = prop;
                    props[prop as usize] += 1;
                    break;
                }
            }
        }
    }
    let mut moved = false;
    for elf in elves {
        let pos = elf.borrow().pos;
        let prop = elf.borrow().prop;
        if prop != i32::MIN {
            if props[prop as usize] == 1 {
                moved = true;
                grid[pos as usize] = None;
                grid[prop as usize] = Some(elf.clone());
                elf.borrow_mut().pos = prop;
            }
            props[prop as usize] = 0;
            elf.borrow_mut().prop = i32::MIN;
        }
    }
    moved
}

pub fn part1(input: &str) -> usize {
    let (elves, mut grid, mut props) = parse(input);
    for i in 0..10 {
        step(&elves, &mut grid, &mut props, i % 4);
    }
    let (mut min_x, mut min_y) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);
    for elf in elves.iter() {
        let (x, y) = (elf.borrow().pos % SZ, elf.borrow().pos / SZ);
        min_x = min(min_x, x);
        min_y = min(min_y, y);
        max_x = max(max_x, x + 1);
        max_y = max(max_y, y + 1);
    }
    (max_x - min_x) as usize * (max_y - min_y) as usize - elves.len()
}

pub fn part2(input: &str) -> usize {
    let (elves, mut grid, mut props) = parse(input);
    for i in 0.. {
        if !step(&elves, &mut grid, &mut props, i % 4) {
            return i + 1;
        }
    }
    unreachable!()
}
